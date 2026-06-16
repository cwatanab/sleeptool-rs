//! トレイ右クリックメニュー。

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, SetForegroundWindow, TrackPopupMenu,
    HMENU, MF_CHECKED, MF_DISABLED, MF_GRAYED, MF_SEPARATOR, MF_STRING, MF_UNCHECKED,
    TPM_NONOTIFY, TPM_RETURNCMD,
};

use crate::state::SharedState;

const ID_PAUSE: usize = 2007;
const ID_SETTINGS: usize = 2008;
const ID_QUIT: usize = 2009;

const PAUSE_LABEL: &str = "監視一時停止";
const SETTINGS_LABEL: &str = "設定...";
const QUIT_LABEL: &str = "終了";

unsafe fn append_item(hmenu: HMENU, id: usize, text: &str, checked: bool, enabled: bool) {
    let mut flags = MF_STRING;
    if checked {
        flags |= MF_CHECKED;
    } else {
        flags |= MF_UNCHECKED;
    }
    if !enabled {
        flags |= MF_GRAYED | MF_DISABLED;
    }
    let text_w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, flags, id, PCWSTR(text_w.as_ptr()));
}

unsafe fn append_separator(hmenu: HMENU) {
    let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR(std::ptr::null()));
}

/// メニューで選ばれた ID。`None` はキャンセル。
pub enum MenuChoice {
    None,
    Pause,
    Settings,
    Quit,
}

pub unsafe fn show(hwnd: HWND, state: &SharedState) -> MenuChoice {
    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);

    let hmenu = CreatePopupMenu().unwrap();
    let paused = {
        let s = state.lock().unwrap();
        s.paused
    };

    append_item(hmenu, ID_PAUSE, PAUSE_LABEL, paused, true);
    append_item(hmenu, ID_SETTINGS, SETTINGS_LABEL, false, true);
    append_separator(hmenu);
    append_item(hmenu, ID_QUIT, QUIT_LABEL, false, true);

    let cmd = TrackPopupMenu(
        hmenu,
        TPM_RETURNCMD | TPM_NONOTIFY,
        pt.x,
        pt.y,
        0,
        hwnd,
        None,
    );
    let cmd_id = cmd.0 as usize;
    let _ = DestroyMenu(hmenu);

    match cmd_id {
        ID_PAUSE => MenuChoice::Pause,
        ID_SETTINGS => MenuChoice::Settings,
        ID_QUIT => MenuChoice::Quit,
        _ => MenuChoice::None,
    }
}
