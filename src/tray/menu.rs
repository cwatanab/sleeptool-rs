//! トレイ右クリックメニュー。

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, DestroyMenu, GetCursorPos, SetForegroundWindow, TrackPopupMenu,
    HMENU, MF_CHECKED, MF_DISABLED, MF_GRAYED, MF_POPUP, MF_SEPARATOR, MF_STRING, MF_UNCHECKED,
    TPM_NONOTIFY, TPM_RETURNCMD,
};

use crate::config::Config;
use crate::state::SharedState;

const ID_PAUSE: usize = 2007;
const ID_QUIT: usize = 2009;

const PAUSE_LABEL: &str = "監視一時停止";
const QUIT_LABEL: &str = "終了";

unsafe fn append_item(hmenu: HMENU, id: usize, text: &str, checked: bool, enabled: bool) {
    let mut flags = MF_STRING;
    if checked { flags |= MF_CHECKED; } else { flags |= MF_UNCHECKED; }
    if !enabled { flags |= MF_GRAYED | MF_DISABLED; }
    let text_w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, flags, id, PCWSTR(text_w.as_ptr()));
}

unsafe fn append_separator(hmenu: HMENU) {
    let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR(std::ptr::null()));
}

pub enum MenuChoice {
    None,
    Pause,
    Quit,
    SetCpu(Option<f64>),
    SetNetwork(Option<f64>),
    SetDiskWrite(Option<f64>),
    SetSleepDelay(u64),
    Toggle(Toggle),
}

#[derive(Clone, Copy)]
pub enum Toggle {
    Hibernate,
    WarnBeforeSleep,
    DisplayOffOnSleep,
    SoundMonitor,
}

struct Preset {
    id: usize,
    label: &'static str,
    value: Option<f64>,
}

const CPU_PRESETS: &[Preset] = &[
    Preset { id: 2101, label: "無効", value: None },
    Preset { id: 2102, label: "1%", value: Some(1.0) },
    Preset { id: 2103, label: "3%", value: Some(3.0) },
    Preset { id: 2104, label: "5%", value: Some(5.0) },
    Preset { id: 2105, label: "8%", value: Some(8.0) },
    Preset { id: 2106, label: "10%", value: Some(10.0) },
    Preset { id: 2107, label: "15%", value: Some(15.0) },
    Preset { id: 2108, label: "25%", value: Some(25.0) },
    Preset { id: 2109, label: "50%", value: Some(50.0) },
    Preset { id: 2110, label: "80%", value: Some(80.0) },
];

const NETWORK_PRESETS: &[Preset] = &[
    Preset { id: 2201, label: "無効", value: None },
    Preset { id: 2202, label: "1 KB/s", value: Some(1_000.0) },
    Preset { id: 2203, label: "5 KB/s", value: Some(5_000.0) },
    Preset { id: 2204, label: "10 KB/s", value: Some(10_000.0) },
    Preset { id: 2205, label: "25 KB/s", value: Some(25_000.0) },
    Preset { id: 2206, label: "50 KB/s", value: Some(50_000.0) },
    Preset { id: 2207, label: "100 KB/s", value: Some(100_000.0) },
    Preset { id: 2208, label: "250 KB/s", value: Some(250_000.0) },
    Preset { id: 2209, label: "500 KB/s", value: Some(500_000.0) },
    Preset { id: 2210, label: "1 MB/s", value: Some(1_000_000.0) },
    Preset { id: 2211, label: "5 MB/s", value: Some(5_000_000.0) },
    Preset { id: 2212, label: "10 MB/s", value: Some(10_000_000.0) },
    Preset { id: 2213, label: "50 MB/s", value: Some(50_000_000.0) },
];

const SLEEP_DELAY_PRESETS: &[Preset] = &[
    Preset { id: 2501, label: "1分", value: Some(60.0) },
    Preset { id: 2502, label: "2分", value: Some(120.0) },
    Preset { id: 2503, label: "5分", value: Some(300.0) },
    Preset { id: 2504, label: "10分", value: Some(600.0) },
    Preset { id: 2505, label: "15分", value: Some(900.0) },
    Preset { id: 2506, label: "20分", value: Some(1200.0) },
    Preset { id: 2507, label: "30分", value: Some(1800.0) },
    Preset { id: 2508, label: "45分", value: Some(2700.0) },
    Preset { id: 2509, label: "60分", value: Some(3600.0) },
    Preset { id: 2510, label: "120分", value: Some(7200.0) },
];

const DISK_WRITE_PRESETS: &[Preset] = &[
    Preset { id: 2301, label: "無効", value: None },
    Preset { id: 2302, label: "10 KB/s", value: Some(10_000.0) },
    Preset { id: 2303, label: "50 KB/s", value: Some(50_000.0) },
    Preset { id: 2304, label: "100 KB/s", value: Some(100_000.0) },
    Preset { id: 2305, label: "250 KB/s", value: Some(250_000.0) },
    Preset { id: 2306, label: "500 KB/s", value: Some(500_000.0) },
    Preset { id: 2307, label: "1 MB/s", value: Some(1_000_000.0) },
    Preset { id: 2308, label: "5 MB/s", value: Some(5_000_000.0) },
    Preset { id: 2309, label: "10 MB/s", value: Some(10_000_000.0) },
    Preset { id: 2310, label: "50 MB/s", value: Some(50_000_000.0) },
    Preset { id: 2311, label: "100 MB/s", value: Some(100_000_000.0) },
    Preset { id: 2312, label: "500 MB/s", value: Some(500_000_000.0) },
];

fn matches_preset(enabled: bool, threshold: f64, presets: &[Preset]) -> usize {
    for p in presets {
        match p.value {
            None => { if !enabled { return p.id; } }
            Some(v) => { if enabled && threshold == v { return p.id; } }
        }
    }
    0
}

unsafe fn build_submenu(hmenu: HMENU, label: &str, presets: &[Preset], checked_id: usize) {
    let sub = CreatePopupMenu().unwrap();
    for p in presets {
        append_item(sub, p.id, p.label, p.id == checked_id, true);
    }
    let label_w: Vec<u16> = label.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, MF_STRING | MF_POPUP, sub.0 as usize, PCWSTR(label_w.as_ptr()));
}

const OPT_ITEMS: &[(usize, &str, Toggle)] = &[
    (2401, "休止状態を使う",           Toggle::Hibernate),
    (2402, "スリープ前に警告",          Toggle::WarnBeforeSleep),
    (2405, "スリープ時ディスプレイOFF",  Toggle::DisplayOffOnSleep),
    (2406, "音声モニター",             Toggle::SoundMonitor),
];

fn opts_checked(config: &Config) -> [bool; 4] {
    [
        config.sleep.hibernate,
        config.sleep.warn_before_sleep,
        config.general.display_off_on_sleep,
        config.sound.enabled,
    ]
}

pub unsafe fn show(hwnd: HWND, state: &SharedState) -> MenuChoice {
    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    let _ = SetForegroundWindow(hwnd);

    let hmenu = CreatePopupMenu().unwrap();
    let (paused, config) = {
        let s = state.lock().unwrap();
        (s.paused, s.config.clone())
    };

    append_item(hmenu, ID_PAUSE, PAUSE_LABEL, paused, true);
    append_separator(hmenu);

    let cpu_checked = matches_preset(config.cpu.enabled, config.cpu.threshold, CPU_PRESETS);
    build_submenu(hmenu, "CPU 使用率", CPU_PRESETS, cpu_checked);

    let net_checked = matches_preset(config.network.enabled, config.network.threshold, NETWORK_PRESETS);
    build_submenu(hmenu, "ネットワーク", NETWORK_PRESETS, net_checked);

    let disk_checked = matches_preset(config.disk.write_enabled, config.disk.write_threshold, DISK_WRITE_PRESETS);
    build_submenu(hmenu, "ディスク書込", DISK_WRITE_PRESETS, disk_checked);

    let sleep_checked = matches_preset(true, config.sleep.delay_seconds as f64, SLEEP_DELAY_PRESETS);
    build_submenu(hmenu, "スリープ待機時間", SLEEP_DELAY_PRESETS, sleep_checked);

    append_separator(hmenu);
    let opt_sub = CreatePopupMenu().unwrap();
    let checked = opts_checked(&config);
    for (i, &(id, label, _)) in OPT_ITEMS.iter().enumerate() {
        append_item(opt_sub, id, label, checked[i], true);
    }
    let opt_w: Vec<u16> = "オプション".encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, MF_STRING | MF_POPUP, opt_sub.0 as usize, PCWSTR(opt_w.as_ptr()));

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

    for p in CPU_PRESETS { if p.id == cmd_id { return MenuChoice::SetCpu(p.value); } }
    for p in NETWORK_PRESETS { if p.id == cmd_id { return MenuChoice::SetNetwork(p.value); } }
    for p in DISK_WRITE_PRESETS { if p.id == cmd_id { return MenuChoice::SetDiskWrite(p.value); } }
    for p in SLEEP_DELAY_PRESETS { if p.id == cmd_id { return MenuChoice::SetSleepDelay(p.value.unwrap() as u64); } }

    for &(id, _, ref toggle) in OPT_ITEMS {
        if id == cmd_id { return MenuChoice::Toggle(*toggle); }
    }

    match cmd_id {
        ID_PAUSE => MenuChoice::Pause,
        ID_QUIT => MenuChoice::Quit,
        _ => MenuChoice::None,
    }
}
