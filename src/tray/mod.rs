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
    WM_RBUTTONUP, WM_USER, WS_POPUP,
};

use crate::config::Config;
use crate::monitors::InhibitFactor;
use crate::platform_win32::WindowsPlatform;
use crate::state::SharedState;

pub const WM_TRAYICON: u32 = WM_USER + 1;
pub const WM_UPDATE_TRAY: u32 = WM_USER + 2;

use icon::IconSet;

struct TrayContext {
    state: SharedState,
    running: Arc<AtomicBool>,
    platform: Arc<WindowsPlatform>,
    hwnd: HWND,
    icons: IconSet,
    current_hicon: windows::Win32::UI::WindowsAndMessaging::HICON,
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
                hIcon: ctx.icons.default,
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
        WM_UPDATE_TRAY => {
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let ctx = &mut *(ctx_ptr as *mut TrayContext);
                let (current_factor, paused, display_state_by_icon) = {
                    let s = ctx.state.lock().unwrap();
                    (s.current_factor, s.paused, s.config.general.display_state_by_icon)
                };

                let target = ctx.icons.pick(current_factor, paused);
                ctx.current_hicon = if display_state_by_icon {
                    target
                } else if paused {
                    ctx.icons.paused
                } else {
                    ctx.icons.default
                };

                let tooltip = tooltip_text(current_factor, paused);

                let mut nid = NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    uFlags: NIF_ICON | NIF_TIP,
                    hIcon: ctx.current_hicon,
                    ..Default::default()
                };
                icon::set_tooltip(&mut nid, tooltip);
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
                    let _ = s.config.save(&Config::config_path());
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
            let _ = s.config.save(&Config::config_path());
            let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
        }
        menu::MenuChoice::Settings => {
            let state = ctx.state.clone();
            let platform = ctx.platform.clone();
            let hwnd_isize = hwnd.0 as isize;
            drop(ctx.state.lock().unwrap()); // release lock before re-entrant call
            crate::settings_gui::show_settings_window(state, platform, Some(hwnd_isize));
        }
        menu::MenuChoice::Quit => {
            ctx.running.store(false, Ordering::Relaxed);
            let _ = DestroyWindow(hwnd);
        }
    }
}

fn tooltip_text(factor: Option<InhibitFactor>, paused: bool) -> &'static str {
    if paused {
        return "SleepTool Rust (一時停止中)";
    }
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

        let icons = IconSet::load()?;
        let default_hicon = icons.default;

        let ctx_box = Box::new(TrayContext {
            state: state.clone(),
            running: running.clone(),
            platform: platform.clone(),
            hwnd: HWND::default(),
            icons,
            current_hicon: default_hicon,
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
