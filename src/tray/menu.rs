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
    Toggle(Toggle),
}

pub enum Toggle {
    Hibernate,
    WarnBeforeSleep,
    DisplayOffOnSleep,
    SoundMonitor,
}

impl Clone for Toggle {
    fn clone(&self) -> Self {
        match self {
            Toggle::Hibernate => Toggle::Hibernate,
            Toggle::WarnBeforeSleep => Toggle::WarnBeforeSleep,
            Toggle::DisplayOffOnSleep => Toggle::DisplayOffOnSleep,
            Toggle::SoundMonitor => Toggle::SoundMonitor,
        }
    }
}

struct Preset {
    id: usize,
    label: &'static str,
    value: Option<f64>,
}

fn cpu_presets() -> Vec<Preset> {
    vec![
        Preset { id: 2101, label: "無効",  value: None },
        Preset { id: 2102, label: "1%",    value: Some(1.0) },
        Preset { id: 2103, label: "5%",    value: Some(5.0) },
        Preset { id: 2104, label: "10%",   value: Some(10.0) },
        Preset { id: 2105, label: "15%",   value: Some(15.0) },
        Preset { id: 2106, label: "25%",   value: Some(25.0) },
        Preset { id: 2107, label: "40%",   value: Some(40.0) },
        Preset { id: 2108, label: "60%",   value: Some(60.0) },
        Preset { id: 2109, label: "80%",   value: Some(80.0) },
    ]
}

fn network_presets() -> Vec<Preset> {
    vec![
        Preset { id: 2201, label: "無効",       value: None },
        Preset { id: 2202, label: "1 KB/s",     value: Some(1_000.0) },
        Preset { id: 2203, label: "10 KB/s",    value: Some(10_000.0) },
        Preset { id: 2204, label: "50 KB/s",    value: Some(50_000.0) },
        Preset { id: 2205, label: "100 KB/s",   value: Some(100_000.0) },
        Preset { id: 2206, label: "500 KB/s",   value: Some(500_000.0) },
        Preset { id: 2207, label: "1 MB/s",     value: Some(1_000_000.0) },
        Preset { id: 2208, label: "5 MB/s",     value: Some(5_000_000.0) },
        Preset { id: 2209, label: "10 MB/s",    value: Some(10_000_000.0) },
    ]
}

fn disk_write_presets() -> Vec<Preset> {
    vec![
        Preset { id: 2301, label: "無効",       value: None },
        Preset { id: 2302, label: "10 KB/s",    value: Some(10_000.0) },
        Preset { id: 2303, label: "100 KB/s",   value: Some(100_000.0) },
        Preset { id: 2304, label: "500 KB/s",   value: Some(500_000.0) },
        Preset { id: 2305, label: "1 MB/s",     value: Some(1_000_000.0) },
        Preset { id: 2306, label: "5 MB/s",     value: Some(5_000_000.0) },
        Preset { id: 2307, label: "10 MB/s",    value: Some(10_000_000.0) },
        Preset { id: 2308, label: "50 MB/s",    value: Some(50_000_000.0) },
        Preset { id: 2309, label: "100 MB/s",   value: Some(100_000_000.0) },
    ]
}

fn matches_preset(enabled: bool, threshold: f64, presets: &[Preset]) -> usize {
    for p in presets {
        match p.value {
            None => { if !enabled { return p.id; } }
            Some(v) => { if enabled && threshold == v { return p.id; } }
        }
    }
    0
}

fn cpu_enabled(config: &Config) -> bool { config.cpu.enabled }
fn cpu_threshold(config: &Config) -> f64 { config.cpu.threshold }
fn net_enabled(config: &Config) -> bool { config.network.enabled }
fn net_threshold(config: &Config) -> f64 { config.network.threshold }
fn disk_enabled(config: &Config) -> bool { config.disk.write_enabled }
fn disk_threshold(config: &Config) -> f64 { config.disk.write_threshold }

unsafe fn build_submenu(hmenu: HMENU, label: &str, presets: &[Preset], checked_id: usize) {
    let sub = CreatePopupMenu().unwrap();
    for p in presets {
        append_item(sub, p.id, p.label, p.id == checked_id, true);
    }
    let label_w: Vec<u16> = label.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, MF_STRING | MF_POPUP, sub.0 as usize, PCWSTR(label_w.as_ptr()));
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

    let cpu = cpu_presets();
    let cpu_checked = matches_preset(cpu_enabled(&config), cpu_threshold(&config), &cpu);
    build_submenu(hmenu, "CPU 使用率", &cpu, cpu_checked);

    let net = network_presets();
    let net_checked = matches_preset(net_enabled(&config), net_threshold(&config), &net);
    build_submenu(hmenu, "ネットワーク", &net, net_checked);

    let disk = disk_write_presets();
    let disk_checked = matches_preset(disk_enabled(&config), disk_threshold(&config), &disk);
    build_submenu(hmenu, "ディスク書込", &disk, disk_checked);

    append_separator(hmenu);
    // オプションサブメニュー
    let opt_sub = CreatePopupMenu().unwrap();
    let opts: [(usize, &str, Toggle, bool); 4] = [
        (2401, "休止状態を使う",           Toggle::Hibernate, config.sleep.hibernate),
        (2402, "スリープ前に警告",          Toggle::WarnBeforeSleep, config.sleep.warn_before_sleep),
        (2405, "スリープ時ディスプレイOFF",  Toggle::DisplayOffOnSleep, config.general.display_off_on_sleep),
        (2406, "音声モニター",             Toggle::SoundMonitor, config.sound.enabled),
    ];
    for &(id, label, _, checked) in &opts {
        append_item(opt_sub, id, label, checked, true);
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

    for p in &cpu {
        if p.id == cmd_id { return MenuChoice::SetCpu(p.value); }
    }
    for p in &net {
        if p.id == cmd_id { return MenuChoice::SetNetwork(p.value); }
    }
    for p in &disk {
        if p.id == cmd_id { return MenuChoice::SetDiskWrite(p.value); }
    }

    for &(id, _, ref toggle, _) in &opts {
        if id == cmd_id { return MenuChoice::Toggle(toggle.clone()); }
    }

    match cmd_id {
        ID_PAUSE => MenuChoice::Pause,
        ID_QUIT => MenuChoice::Quit,
        _ => MenuChoice::None,
    }
}
