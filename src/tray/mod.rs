//! タスクトレイアイコンとメニュー。

mod icon;
mod menu;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, HINSTANCE};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NOTIFYICONDATAW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
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
                let hicon = if display_state_by_icon { target }
                    else if paused { ctx.icons.pick(None, true) }
                    else { ctx.icons.pick(None, false) };

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
                crate::platform_win32::optimize_memory();
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

fn update_config(state: &SharedState, modify: impl FnOnce(&mut crate::config::Config)) {
    let (path, new_config) = {
        let s = state.lock().unwrap();
        let mut cfg = (*s.config).clone();
        modify(&mut cfg);
        (s.config_path.clone(), cfg)
    };
    let _ = new_config.save(&path);
    let mut s = state.lock().unwrap();
    s.config = Arc::new(new_config);
}

unsafe fn handle_menu_choice(hwnd: HWND, ctx: &mut TrayContext, choice: menu::MenuChoice) {
    match choice {
        menu::MenuChoice::None => {}
        menu::MenuChoice::Pause => {
            let mut s = ctx.state.lock().unwrap();
            s.paused = !s.paused;
            let _ = s.config.save(&s.config_path);
            let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
        }
        menu::MenuChoice::Quit => {
            ctx.running.store(false, Ordering::Relaxed);
            let _ = DestroyWindow(hwnd);
        }
        menu::MenuChoice::SetCpu(v) => update_config(&ctx.state, |c| {
            if let Some(t) = v { c.cpu.enabled = true; c.cpu.threshold = t; } else { c.cpu.enabled = false; }
        }),
        menu::MenuChoice::SetNetwork(v) => update_config(&ctx.state, |c| {
            if let Some(t) = v { c.network.enabled = true; c.network.threshold = t; } else { c.network.enabled = false; }
        }),
        menu::MenuChoice::SetDiskWrite(v) => update_config(&ctx.state, |c| {
            if let Some(t) = v { c.disk.write_enabled = true; c.disk.write_threshold = t; } else { c.disk.write_enabled = false; }
        }),
        menu::MenuChoice::SetSleepDelay(v) => update_config(&ctx.state, |c| {
            c.sleep.delay_seconds = v;
        }),
        menu::MenuChoice::Toggle(t) => update_config(&ctx.state, |c| match t {
            menu::Toggle::Hibernate => c.sleep.hibernate = !c.sleep.hibernate,
            menu::Toggle::WarnBeforeSleep => c.sleep.warn_before_sleep = !c.sleep.warn_before_sleep,
            menu::Toggle::DisplayOffOnSleep => c.general.display_off_on_sleep = !c.general.display_off_on_sleep,
            menu::Toggle::SoundMonitor => c.sound.enabled = !c.sound.enabled,
        }),
    }
}

fn tooltip_text(factor: Option<InhibitFactor>, paused: bool, platform: &Arc<WindowsPlatform>) -> String {
    let status = match (paused, factor) {
        (true, _) => "SleepTool Rust (一時停止中)",
        (_, Some(InhibitFactor::Process)) => "SleepTool Rust - プロセス実行中",
        (_, Some(InhibitFactor::Sound)) => "SleepTool Rust - サウンド出力中",
        (_, Some(InhibitFactor::Cpu)) => "SleepTool Rust - CPU使用中",
        (_, Some(InhibitFactor::Network)) => "SleepTool Rust - ネットワーク使用中",
        (_, Some(InhibitFactor::DiskRead)) => "SleepTool Rust - ディスク読み込み中",
        (_, Some(InhibitFactor::DiskWrite)) => "SleepTool Rust - ディスク書き込み中",
        (_, Some(InhibitFactor::Input)) => "SleepTool Rust - 入力検知中",
        (_, None) => "SleepTool Rust",
    };

    match PerformanceProbe::query_performance(platform.as_ref()) {
        Ok(s) => {
            use std::fmt::Write;
            let mut out = String::with_capacity(status.len() + 64);
            out.push_str(status);
            let _ = write!(out, "\n⚡ {:.0}%  🌐 {:.1}KB/s  💾 {:.1}KB/s",
                s.cpu_percent, s.network_bytes_per_sec / 1000.0, s.disk_write_bytes_per_sec / 1000.0);
            out
        }
        Err(_) => status.to_string(),
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
        crate::platform_win32::optimize_memory();

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
