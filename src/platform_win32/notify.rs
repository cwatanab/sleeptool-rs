//! 通知（トレイアイコンのバルーンでスリープ警告を表示）。

use std::sync::atomic::Ordering;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NOTIFYICONDATAW, NIF_INFO, NIIF_INFO, NIM_MODIFY,
};

use crate::error::Result;
use crate::platform::InputProbe;

use super::WindowsPlatform;

const TITLE: &str = "SleepTool";
const BODY: &str = "まもなくスリープに移行します";

pub fn show_sleep_warning(platform: &WindowsPlatform, seconds: u64) -> Result<bool> {
    show_balloon(platform);
    wait_for_input(platform, seconds)
}

fn show_balloon(platform: &WindowsPlatform) {
    let hwnd_val = platform.tray_hwnd.load(Ordering::Relaxed);
    if hwnd_val == 0 { return; }
    unsafe {
        let hwnd = HWND(hwnd_val as *mut std::ffi::c_void);
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: 1,
            uFlags: NIF_INFO,
            dwInfoFlags: NIIF_INFO,
            ..Default::default()
        };

        let title_utf16: Vec<u16> = TITLE.encode_utf16().collect();
        let tlen = title_utf16.len().min(63);
        nid.szInfoTitle[..tlen].copy_from_slice(&title_utf16[..tlen]);

        let msg_utf16: Vec<u16> = BODY.encode_utf16().collect();
        let mlen = msg_utf16.len().min(255);
        nid.szInfo[..mlen].copy_from_slice(&msg_utf16[..mlen]);

        let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
    }
}

fn wait_for_input(platform: &WindowsPlatform, seconds: u64) -> Result<bool> {
    let start_idle = InputProbe::last_input_idle_seconds(platform, false).unwrap_or(0);
    let check_interval = std::time::Duration::from_millis(100);
    let total_ticks = seconds * 10;

    for _ in 0..total_ticks {
        let current_idle = InputProbe::last_input_idle_seconds(platform, false).unwrap_or(0);
        if current_idle < start_idle || current_idle == 0 { return Ok(true); }
        std::thread::sleep(check_interval);
    }
    Ok(false)
}
