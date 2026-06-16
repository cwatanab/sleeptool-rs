//! 自動起動 (HKCU\...\Run) の登録 / 解除 / 確認。
//!
//! `Run` キーに `"<exe 絶対パス>"` を書き込む。
//! 値が存在するかどうかで `is_auto_start_enabled` を判定する。

use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
};

use crate::error::Result;

use super::util::{check_win32, to_wstring};

const SUBKEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "SleepToolRust";

pub fn set_auto_start(enable: bool) -> Result<()> {
    unsafe {
        let subkey = to_wstring(SUBKEY);
        let value_name = to_wstring(VALUE_NAME);

        if enable {
            let exe_path = std::env::current_exe()?;
            let path_str = format!("\"{}\"", exe_path.display());
            let path_wstr = to_wstring(&path_str);

            let mut key = HKEY::default();
            check_win32(RegCreateKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                None,
                &mut key,
                None,
            ))?;

            let bytes = std::slice::from_raw_parts(
                path_wstr.as_ptr() as *const u8,
                path_wstr.len() * std::mem::size_of::<u16>(),
            );
            check_win32(RegSetValueExW(
                key,
                PCWSTR(value_name.as_ptr()),
                0,
                REG_SZ,
                Some(bytes),
            ))?;
            let _ = RegCloseKey(key);
        } else {
            let mut key = HKEY::default();
            let result = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(subkey.as_ptr()),
                0,
                KEY_WRITE,
                &mut key,
            );
            if check_win32(result).is_ok() {
                let _ = RegDeleteValueW(key, PCWSTR(value_name.as_ptr()));
                let _ = RegCloseKey(key);
            }
        }
        Ok(())
    }
}

pub fn is_auto_start_enabled() -> Result<bool> {
    unsafe {
        let subkey = to_wstring(SUBKEY);
        let value_name = to_wstring(VALUE_NAME);

        let mut key = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut key,
        );
        if check_win32(result).is_err() {
            return Ok(false);
        }

        let mut data_type = REG_SZ;
        let mut data = [0u8; 1024];
        let mut data_len = data.len() as u32;
        let query_result = RegQueryValueExW(
            key,
            PCWSTR(value_name.as_ptr()),
            None,
            Some(&mut data_type),
            Some(data.as_mut_ptr()),
            Some(&mut data_len),
        );
        let _ = RegCloseKey(key);

        Ok(check_win32(query_result).is_ok())
    }
}
