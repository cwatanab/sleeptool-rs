//! 電源制御（スリープ / ハイバネート / ディスプレイオフ）。

use windows::Win32::Foundation::{BOOLEAN, HWND, LPARAM, WPARAM};
use windows::Win32::System::Power::SetSuspendState;
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND};

use crate::error::Result;
use crate::platform::SleepType;

use super::util::check_boolean;

pub fn suspend(sleep_type: SleepType, force: bool) -> Result<()> {
    unsafe {
        let hibernate = sleep_type == SleepType::Hibernate;
        let result = SetSuspendState(
            BOOLEAN(if hibernate { 1 } else { 0 }),
            BOOLEAN(if force { 1 } else { 0 }),
            BOOLEAN(0),
        );
        check_boolean(result)?;
        Ok(())
    }
}

pub fn turn_display_off() -> Result<()> {
    unsafe {
        SendMessageW(
            HWND(std::ptr::null_mut()),
            WM_SYSCOMMAND,
            WPARAM(SC_MONITORPOWER as usize),
            LPARAM(2),
        );
        Ok(())
    }
}
