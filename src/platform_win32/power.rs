//! 電源制御（スリープ / ハイバネート / ディスプレイオフ）。

use windows::Win32::Foundation::{BOOLEAN, HWND, LPARAM, WPARAM};
use windows::Win32::System::Power::SetSuspendState;
use windows::Win32::UI::WindowsAndMessaging::{SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND};

use crate::error::Result;
use crate::platform::SleepType;

use super::util::check_boolean;

pub fn suspend(sleep_type: SleepType, force: bool) -> Result<()> {
    unsafe {
        check_boolean(SetSuspendState(
            BOOLEAN((sleep_type == SleepType::Hibernate) as u8),
            BOOLEAN(force as u8),
            BOOLEAN(0),
        ))
    }
}

pub fn turn_display_off() -> Result<()> {
    unsafe {
        SendMessageW(HWND(std::ptr::null_mut()), WM_SYSCOMMAND, WPARAM(SC_MONITORPOWER as usize), LPARAM(2));
        Ok(())
    }
}
