//! Windows API 呼び出しのエラーチェックと変換ヘルパ。

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::Win32::Foundation::{BOOL, BOOLEAN};

use crate::error::{Result, SleepToolError};

pub fn check_pdh(status: u32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform(format!("PDH error: 0x{:08X}", status)))
    }
}

pub fn check_bool(result: BOOL) -> Result<()> {
    if result.as_bool() {
        Ok(())
    } else {
        Err(SleepToolError::Platform("Windows API returned false".to_string()))
    }
}

pub fn check_boolean(result: BOOLEAN) -> Result<()> {
    if result.0 != 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform("Windows API returned false".to_string()))
    }
}

pub fn win_result<T>(result: windows::core::Result<T>) -> Result<T> {
    result.map_err(|e| SleepToolError::Platform(e.to_string()))
}

pub fn check_win32(status: windows::Win32::Foundation::WIN32_ERROR) -> Result<()> {
    if status.0 == 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform(format!("Windows error: 0x{:08X}", status.0)))
    }
}

pub fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}


