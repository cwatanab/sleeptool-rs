//! Windows のライト/ダークテーマに応じたコンテキストメニュー対応。
//!
//! Windows 10 (1809以降) および Windows 11 の `uxtheme.dll` の
//! 非公開 API (SetPreferredAppMode, AllowDarkModeForWindow, FlushMenuThemes)
//! を動的にロードし、アプリのテーマ設定に合わせて Win32 メニューを表示します。

use std::sync::OnceLock;
use windows::core::PCSTR;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ,
    REG_DWORD, REG_VALUE_TYPE,
};

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PreferredAppMode {
    Default = 0,
    AllowDark = 1,
    ForceDark = 2,
    ForceLight = 3,
    Max = 4,
}

type FnSetPreferredAppMode = unsafe extern "system" fn(PreferredAppMode) -> i32;
type FnAllowDarkModeForWindow = unsafe extern "system" fn(HWND, BOOL) -> BOOL;
type FnFlushMenuThemes = unsafe extern "system" fn();

#[derive(Clone, Copy)]
struct DarkModeApi {
    set_preferred_app_mode: Option<FnSetPreferredAppMode>,
    allow_dark_mode_for_window: Option<FnAllowDarkModeForWindow>,
    flush_menu_themes: Option<FnFlushMenuThemes>,
}

static API: OnceLock<DarkModeApi> = OnceLock::new();

fn get_api() -> &'static DarkModeApi {
    API.get_or_init(|| unsafe {
        let mut api = DarkModeApi {
            set_preferred_app_mode: None,
            allow_dark_mode_for_window: None,
            flush_menu_themes: None,
        };

        let uxtheme = LoadLibraryA(PCSTR(b"uxtheme.dll\0".as_ptr()));
        if let Ok(hmodule) = uxtheme {
            // Ordinal 135: SetPreferredAppMode (Windows 10 1903+)
            if let Some(p) = GetProcAddress(hmodule, PCSTR(135 as usize as *const u8)) {
                api.set_preferred_app_mode = Some(std::mem::transmute(p));
            }

            // Ordinal 133: AllowDarkModeForWindow
            if let Some(p) = GetProcAddress(hmodule, PCSTR(133 as usize as *const u8)) {
                api.allow_dark_mode_for_window = Some(std::mem::transmute(p));
            }

            // Ordinal 136: FlushMenuThemes
            if let Some(p) = GetProcAddress(hmodule, PCSTR(136 as usize as *const u8)) {
                api.flush_menu_themes = Some(std::mem::transmute(p));
            }
        }

        api
    })
}

/// OS のテーマ設定（ダークモード: 0, ライトモード: 1）を判定
#[allow(dead_code)]
pub fn is_system_dark_mode() -> bool {
    is_dark_mode("SystemUsesLightTheme")
}

fn is_dark_mode(value: &str) -> bool {
    unsafe {
        let subkey: Vec<u16> = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\0"
            .encode_utf16()
            .collect();
        let value_name: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

        let mut hkey = HKEY::default();
        if RegOpenKeyExW(
            HKEY_CURRENT_USER,
            windows::core::PCWSTR(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        )
        .is_ok()
        {
            let mut data: u32 = 0;
            let mut data_size = std::mem::size_of::<u32>() as u32;
            let mut val_type = REG_VALUE_TYPE(0);
            let res = RegQueryValueExW(
                hkey,
                windows::core::PCWSTR(value_name.as_ptr()),
                None,
                Some(&mut val_type),
                Some(&mut data as *mut u32 as *mut u8),
                Some(&mut data_size),
            );
            let _ = RegCloseKey(hkey);

            if res.is_ok() && val_type == REG_DWORD {
                return data == 0;
            }
        }
    }
    false
}

use windows::core::{w, PCWSTR};
use windows::Win32::UI::Controls::SetWindowTheme;
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE};

/// Clipper と同様、ウィンドウ作成前にアプリのダークモードを許可する。
pub unsafe fn initialize_app_theme() {
    if let Some(set_mode) = get_api().set_preferred_app_mode {
        set_mode(PreferredAppMode::AllowDark);
    }
}

/// ウィンドウおよびアプリ全体のテーマモードを設定・更新
pub unsafe fn setup_theme_for_window(hwnd: HWND) {
    let api = get_api();

    let is_dark = is_dark_mode("AppsUseLightTheme");
    let dark = BOOL(is_dark as i32);
    let _ = DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &dark as *const BOOL as *const _, std::mem::size_of::<BOOL>() as u32);

    if let Some(allow_window) = api.allow_dark_mode_for_window {
        let _ = allow_window(hwnd, dark);
    }

    // Clipper と同様、Explorer テーマをウィンドウへ適用する。
    if is_dark {
        let _ = SetWindowTheme(hwnd, w!("DarkMode_Explorer"), PCWSTR::null());
    } else {
        let _ = SetWindowTheme(hwnd, w!("Explorer"), PCWSTR::null());
    }

    if let Some(flush) = api.flush_menu_themes {
        flush();
    }
}

/// 設定変更 (WM_SETTINGCHANGE) 時にメニューのテーマキャッシュを破棄
pub unsafe fn refresh_menu_theme(hwnd: HWND) {
    setup_theme_for_window(hwnd);
}
