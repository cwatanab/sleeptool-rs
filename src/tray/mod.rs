//! タスクトレイアイコンとメニュー。
//!
//! 構成:
//! - `icon`: アイコン画像の管理
//! - `menu`: 右クリックメニュー
//!
//! エントリポイントは `run_tray`。内部の `wnd_proc` が WM_TRAYICON /
//! WM_UPDATE_TRAY / WM_LBUTTONDBLCLK などを処理する。

mod icon;
mod menu;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, HINSTANCE};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NOTIFYICONDATAW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE,
    NIM_MODIFY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
    HMENU, PostMessageW, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, TranslateMessage,
    CREATESTRUCTW, GWLP_USERDATA, MSG, WNDCLASSW, WM_CREATE, WM_DESTROY, WM_LBUTTONDBLCLK,
    WM_RBUTTONUP, WM_SETTINGCHANGE, WM_USER, WS_POPUP,
};

use crate::monitors::InhibitFactor;
use crate::platform::PerformanceProbe;
use crate::platform_win32::WindowsPlatform;
use crate::state::SharedState;

pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const WM_UPDATE_TRAY: u32 = WM_USER + 2;

use icon::ThemeIconSet;

struct TrayContext {
    state: SharedState,
    running: Arc<AtomicBool>,
    platform: Arc<WindowsPlatform>,
    hwnd: HWND,
    icons: ThemeIconSet,
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let create_struct = &*(lparam.0 as *const CREATESTRUCTW);
            let ctx_ptr = create_struct.lpCreateParams as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ctx_ptr);

            let ctx = &mut *(ctx_ptr as *mut TrayContext);
            ctx.hwnd = hwnd;
            ctx.platform.tray_hwnd.store(hwnd.0 as isize, Ordering::Relaxed);

            let mut nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
                uCallbackMessage: WM_TRAYICON,
                hIcon: ctx.icons.pick(None, false),
                ..Default::default()
            };
            icon::set_tooltip(&mut nid, "SleepTool Rust");
            let _ = Shell_NotifyIconW(NIM_ADD, &nid);

            return LRESULT(0);
        }
        WM_DESTROY => {
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let nid = NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    ..Default::default()
                };
                let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
                let _ctx_box = Box::from_raw(ctx_ptr as *mut TrayContext);
            }
            PostQuitMessage(0);
            return LRESULT(0);
        }
        WM_SETTINGCHANGE => {
            icon::update_dark_mode();
            let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
            return LRESULT(0);
        }
        WM_UPDATE_TRAY => {
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let ctx = &mut *(ctx_ptr as *mut TrayContext);
                let (current_factor, paused, display_state_by_icon) = {
                    let s = ctx.state.lock().unwrap();
                    (s.current_factor, s.paused, s.config.general.display_state_by_icon)
                };

                let target = ctx.icons.pick(current_factor, paused);
                let hicon = if display_state_by_icon {
                    target
                } else if paused {
                    ctx.icons.pick(None, true)
                } else {
                    ctx.icons.pick(None, false)
                };

                let tooltip = tooltip_text(current_factor, paused, &ctx.platform);

                let mut nid = NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    uFlags: NIF_ICON | NIF_TIP,
                    hIcon: hicon,
                    ..Default::default()
                };
                icon::set_tooltip(&mut nid, &tooltip);
                let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
            }
            return LRESULT(0);
        }
        WM_TRAYICON => {
            let event_id = lparam.0 as u32;
            if event_id == WM_LBUTTONDBLCLK {
                let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
                if ctx_ptr != 0 {
                    let ctx = &mut *(ctx_ptr as *mut TrayContext);
                    let mut s = ctx.state.lock().unwrap();
                    s.paused = !s.paused;
                    let _ = s.config.save(&s.config_path.clone());
                    let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
                }
                return LRESULT(0);
            }
            if event_id == WM_RBUTTONUP {
                let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
                if ctx_ptr != 0 {
                    let ctx = &mut *(ctx_ptr as *mut TrayContext);
                    let choice = menu::show(hwnd, &ctx.state);
                    handle_menu_choice(hwnd, ctx, choice);
                }
                return LRESULT(0);
            }
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

unsafe fn handle_menu_choice(hwnd: HWND, ctx: &mut TrayContext, choice: menu::MenuChoice) {
    match choice {
        menu::MenuChoice::None => {}
        menu::MenuChoice::Pause => {
            let mut s = ctx.state.lock().unwrap();
            s.paused = !s.paused;
            let _ = s.config.save(&s.config_path.clone());
            let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
        }
        menu::MenuChoice::Quit => {
            ctx.running.store(false, Ordering::Relaxed);
            let _ = DestroyWindow(hwnd);
        }
        menu::MenuChoice::SetCpu(value) => {
            let (config_path, new_config) = {
                let s = ctx.state.lock().unwrap();
                let mut cfg = (*s.config).clone();
                if let Some(v) = value {
                    cfg.cpu.enabled = true;
                    cfg.cpu.threshold = v;
                } else {
                    cfg.cpu.enabled = false;
                }
                (s.config_path.clone(), cfg)
            };
            let _ = new_config.save(&config_path);
            let mut s = ctx.state.lock().unwrap();
            s.config = Arc::new(new_config);
        }
        menu::MenuChoice::SetNetwork(value) => {
            let (config_path, new_config) = {
                let s = ctx.state.lock().unwrap();
                let mut cfg = (*s.config).clone();
                if let Some(v) = value {
                    cfg.network.enabled = true;
                    cfg.network.threshold = v;
                } else {
                    cfg.network.enabled = false;
                }
                (s.config_path.clone(), cfg)
            };
            let _ = new_config.save(&config_path);
            let mut s = ctx.state.lock().unwrap();
            s.config = Arc::new(new_config);
        }
        menu::MenuChoice::SetDiskWrite(value) => {
            let (config_path, new_config) = {
                let s = ctx.state.lock().unwrap();
                let mut cfg = (*s.config).clone();
                if let Some(v) = value {
                    cfg.disk.write_enabled = true;
                    cfg.disk.write_threshold = v;
                } else {
                    cfg.disk.write_enabled = false;
                }
                (s.config_path.clone(), cfg)
            };
            let _ = new_config.save(&config_path);
            let mut s = ctx.state.lock().unwrap();
            s.config = Arc::new(new_config);
        }
        menu::MenuChoice::Toggle(toggle) => {
            let (config_path, new_config) = {
                let s = ctx.state.lock().unwrap();
                let mut cfg = (*s.config).clone();
                match toggle {
                    menu::Toggle::Hibernate => cfg.sleep.hibernate = !cfg.sleep.hibernate,
                    menu::Toggle::WarnBeforeSleep => cfg.sleep.warn_before_sleep = !cfg.sleep.warn_before_sleep,
                    menu::Toggle::DisplayOffOnSleep => cfg.general.display_off_on_sleep = !cfg.general.display_off_on_sleep,
                    menu::Toggle::SoundMonitor => cfg.sound.enabled = !cfg.sound.enabled,
                }
                (s.config_path.clone(), cfg)
            };
            let _ = new_config.save(&config_path);
            let mut s = ctx.state.lock().unwrap();
            s.config = Arc::new(new_config);
        }
    }
}

fn tooltip_text(factor: Option<InhibitFactor>, paused: bool, platform: &Arc<WindowsPlatform>) -> String {
    let status = if paused {
        "SleepTool Rust (一時停止中)"
    } else {
        match factor {
            Some(InhibitFactor::Process) => "SleepTool Rust - プロセス実行中",
            Some(InhibitFactor::Sound) => "SleepTool Rust - サウンド出力中",
            Some(InhibitFactor::Cpu) => "SleepTool Rust - CPU使用中",
            Some(InhibitFactor::Network) => "SleepTool Rust - ネットワーク使用中",
            Some(InhibitFactor::DiskRead) => "SleepTool Rust - ディスク読み込み中",
            Some(InhibitFactor::DiskWrite) => "SleepTool Rust - ディスク書き込み中",
            Some(InhibitFactor::Input) => "SleepTool Rust - 入力検知中",
            None => "SleepTool Rust",
        }
    };

    if let Ok(snapshot) = PerformanceProbe::query_performance(platform.as_ref()) {
        format!(
            "{}\n⚡CPU:{:.0}%  🌐Network:{:.1}KB/s  💾Disk:{:.1}KB/s",
            status,
            snapshot.cpu_percent,
            snapshot.network_bytes_per_sec / 1000.0,
            snapshot.disk_write_bytes_per_sec / 1000.0,
        )
    } else {
        status.to_string()
    }
}

pub fn run_tray(state: SharedState, running: Arc<AtomicBool>, platform: Arc<WindowsPlatform>) -> anyhow::Result<()> {
    unsafe {
        let class_name: Vec<u16> = "SleepToolTrayWindowClass\0".encode_utf16().collect();
        let hinstance = HINSTANCE(windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap_or_default().0);

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        let icons = ThemeIconSet::load()?;

        let ctx_box = Box::new(TrayContext {
            state: state.clone(),
            running: running.clone(),
            platform: platform.clone(),
            hwnd: HWND::default(),
            icons,
        });

        let ctx_ptr = Box::into_raw(ctx_box);

        let window_name: Vec<u16> = "SleepToolTrayWindow\0".encode_utf16().collect();
        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(window_name.as_ptr()),
            WS_POPUP,
            0, 0, 0, 0,
            HWND::default(),
            HMENU::default(),
            hinstance,
            Some(ctx_ptr as *const std::ffi::c_void),
        );

        if hwnd.is_err() {
            let _ = Box::from_raw(ctx_ptr);
            anyhow::bail!("CreateWindowExW failed");
        }
        let hwnd = hwnd.unwrap();

        {
            let mut s = state.lock().unwrap();
            s.hwnd = Some(hwnd.0 as isize);
        }

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = windows::Win32::UI::WindowsAndMessaging::UnregisterClassW(
            PCWSTR(class_name.as_ptr()),
            hinstance,
        );
    }

    Ok(())
}
