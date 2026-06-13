use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::config::Config;
use crate::monitors::InhibitFactor;
use crate::state::SharedState;
use crate::platform_win32::WindowsPlatform;
use crate::platform::Platform;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, LRESULT, POINT, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::{
    RegisterClassW, CreateWindowExW, DefWindowProcW, PostQuitMessage, DestroyWindow,
    CreatePopupMenu, AppendMenuW, TrackPopupMenu, DestroyMenu, GetCursorPos, SetForegroundWindow,
    CreateIconIndirect, PostMessageW, DispatchMessageW, TranslateMessage, GetMessageW,
    DestroyIcon,
    WNDCLASSW, MSG, ICONINFO, HMENU, HICON,
    GWLP_USERDATA, SetWindowLongPtrW, GetWindowLongPtrW, CREATESTRUCTW,
    WS_POPUP, WINDOW_EX_STYLE,
    TPM_RETURNCMD, TPM_NONOTIFY,
    WM_CREATE, WM_DESTROY, WM_USER, WM_RBUTTONUP, WM_LBUTTONDBLCLK,
    MF_STRING, MF_CHECKED, MF_UNCHECKED, MF_DISABLED, MF_GRAYED, MF_SEPARATOR, MF_POPUP,
};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NOTIFYICONDATAW,
    NIM_ADD, NIM_MODIFY, NIM_DELETE,
    NIF_ICON, NIF_MESSAGE, NIF_TIP,
};
use windows::Win32::Graphics::Gdi::{
    CreateDIBSection, CreateBitmap, DeleteObject, GetDC, ReleaseDC,
    BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
};

const WM_TRAYICON: u32 = WM_USER + 1;
pub const WM_UPDATE_TRAY: u32 = WM_USER + 2;


const ID_SOUND_ENABLED: usize = 2001;
const ID_DISPLAY_STATE_BY_ICON: usize = 2002;
const ID_WARN_BEFORE_SLEEP: usize = 2003;
const ID_WARN_SOUND: usize = 2004;
const ID_AUTO_START: usize = 2005;
const ID_DISPLAY_OFF_ON_SLEEP: usize = 2006;
const ID_PAUSE: usize = 2007;

const ID_QUIT: usize = 2009;

const ID_DELAY_START: usize = 1000;
const ID_CPU_START: usize = 1100;
const ID_DISK_WRITE_START: usize = 1200;
const ID_NET_START: usize = 1300;
const ID_PROC_START: usize = 1400;

const DELAY_OPTIONS: &[(u64, &str)] = &[
    (60, "1分"),
    (180, "3分"),
    (300, "5分"),
    (600, "10分"),
    (900, "15分"),
    (1800, "30分"),
    (2700, "45分"),
    (3600, "60分"),
    (5400, "90分"),
    (7200, "120分"),
    (10800, "180分"),
];

const CPU_OPTIONS: &[(f64, &str)] = &[
    (0.0, "監視しない"),
    (1.0, "1%"),
    (3.0, "3%"),
    (5.0, "5%"),
    (8.0, "8%"),
    (10.0, "10%"),
    (15.0, "15%"),
    (20.0, "20%"),
    (25.0, "25%"),
    (30.0, "30%"),
    (40.0, "40%"),
    (50.0, "50%"),
];

const DISK_WRITE_OPTIONS: &[(f64, &str)] = &[
    (0.0, "監視しない"),
    (10000.0, "10 KB/s"),
    (100000.0, "100 KB/s"),
    (500000.0, "500 KB/s"),
    (1000000.0, "1 MB/s"),
    (5000000.0, "5 MB/s"),
    (10000000.0, "10 MB/s"),
];

const NET_OPTIONS: &[(f64, &str)] = &[
    (0.0, "監視しない"),
    (1000.0, "1 KB/s"),
    (10000.0, "10 KB/s"),
    (100000.0, "100 KB/s"),
    (500000.0, "500 KB/s"),
    (1000000.0, "1 MB/s"),
    (2000000.0, "2 MB/s"),
    (5000000.0, "5 MB/s"),
    (10000000.0, "10 MB/s"),
];

struct TrayContext {
    state: SharedState,
    running: Arc<AtomicBool>,
    platform: Arc<WindowsPlatform>,
    hwnd: HWND,
    hicon_default: HICON,
    hicon_paused: HICON,
    hicon_cpu: HICON,
    hicon_network: HICON,
    hicon_disk: HICON,
    hicon_sound: HICON,
    hicon_process: HICON,
    hicon_printer: HICON,
    current_hicon: HICON,
}

impl Drop for TrayContext {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyIcon(self.hicon_default);
            let _ = DestroyIcon(self.hicon_paused);
            let _ = DestroyIcon(self.hicon_cpu);
            let _ = DestroyIcon(self.hicon_network);
            let _ = DestroyIcon(self.hicon_disk);
            let _ = DestroyIcon(self.hicon_sound);
            let _ = DestroyIcon(self.hicon_process);
            let _ = DestroyIcon(self.hicon_printer);
        }
    }
}

unsafe fn create_hicon_from_rgba(rgba: &[u8], width: i32, height: i32) -> windows::core::Result<HICON> {
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height, // Negative height means top-down DIB
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0, // BI_RGB
            ..Default::default()
        },
        ..Default::default()
    };

    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let hdc = GetDC(HWND::default());
    let hcolor = CreateDIBSection(
        hdc,
        &bmi,
        DIB_RGB_COLORS,
        &mut bits,
        None,
        0,
    )?;
    ReleaseDC(HWND::default(), hdc);

    let bits_slice = std::slice::from_raw_parts_mut(bits as *mut u8, (width * height * 4) as usize);
    for i in 0..(width * height) as usize {
        let r = rgba[i * 4];
        let g = rgba[i * 4 + 1];
        let b = rgba[i * 4 + 2];
        let a = rgba[i * 4 + 3];
        bits_slice[i * 4] = b;
        bits_slice[i * 4 + 1] = g;
        bits_slice[i * 4 + 2] = r;
        bits_slice[i * 4 + 3] = a;
    }

    let mask_bits = vec![0u8; (width * height / 8) as usize];
    let hmask = CreateBitmap(
        width,
        height,
        1,
        1,
        Some(mask_bits.as_ptr() as *const _),
    );

    let mut icon_info = ICONINFO {
        fIcon: true.into(),
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hmask,
        hbmColor: hcolor,
    };
    
    let hicon = CreateIconIndirect(&mut icon_info)?;

    let _ = DeleteObject(hcolor);
    let _ = DeleteObject(hmask);

    Ok(hicon)
}

fn load_icon_handle(name: &str) -> anyhow::Result<HICON> {
    let bytes: &[u8] = match name {
        "default" => include_bytes!("../assets/icons/default.rgba"),
        "cpu" => include_bytes!("../assets/icons/cpu.rgba"),
        "network" => include_bytes!("../assets/icons/network.rgba"),
        "disk" => include_bytes!("../assets/icons/disk.rgba"),
        "sound" => include_bytes!("../assets/icons/sound.rgba"),
        "process" => include_bytes!("../assets/icons/process.rgba"),
        "printer" => include_bytes!("../assets/icons/printer.rgba"),
        "paused" => include_bytes!("../assets/icons/paused.rgba"),
        _ => include_bytes!("../assets/icons/default.rgba"),
    };
    unsafe { create_hicon_from_rgba(bytes, 32, 32).map_err(|e| anyhow::anyhow!("GDI error: {:?}", e)) }
}

fn factor_icon_name(factor: Option<InhibitFactor>, paused: bool) -> &'static str {
    if paused {
        return "paused";
    }
    match factor {
        Some(InhibitFactor::Printer) => "printer",
        Some(InhibitFactor::Process) => "process",
        Some(InhibitFactor::Sound) => "sound",
        Some(InhibitFactor::Cpu) => "cpu",
        Some(InhibitFactor::Network) => "network",
        Some(InhibitFactor::DiskRead) | Some(InhibitFactor::DiskWrite) => "disk",
        Some(InhibitFactor::Input) | None => "default",
    }
}

fn set_tooltip(nid: &mut NOTIFYICONDATAW, text: &str) {
    let mut tip = [0u16; 128];
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let len = utf16.len().min(127);
    tip[..len].copy_from_slice(&utf16[..len]);
    nid.szTip = tip;
}

unsafe fn append_menu_item(hmenu: HMENU, id: usize, text: &str, checked: bool, enabled: bool) {
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

unsafe fn append_submenu(hmenu: HMENU, hsubmenu: HMENU, text: &str) {
    let text_w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let _ = AppendMenuW(hmenu, MF_POPUP, hsubmenu.0 as usize, PCWSTR(text_w.as_ptr()));
}

unsafe fn append_separator(hmenu: HMENU) {
    let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, PCWSTR(std::ptr::null()));
}

fn find_closest_cpu(val: f64, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }
    let mut best_idx = 1;
    let mut min_diff = f64::MAX;
    for (i, &(opt_val, _)) in CPU_OPTIONS.iter().enumerate() {
        if opt_val == 0.0 {
            continue;
        }
        let diff = (val - opt_val).abs();
        if diff < min_diff {
            min_diff = diff;
            best_idx = i;
        }
    }
    best_idx
}

fn find_closest_disk_write(val: f64, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }
    let mut best_idx = 1;
    let mut min_diff = f64::MAX;
    for (i, &(opt_val, _)) in DISK_WRITE_OPTIONS.iter().enumerate() {
        if opt_val == 0.0 {
            continue;
        }
        let diff = (val - opt_val).abs();
        if diff < min_diff {
            min_diff = diff;
            best_idx = i;
        }
    }
    best_idx
}

fn find_closest_net(val: f64, enabled: bool) -> usize {
    if !enabled {
        return 0;
    }
    let mut best_idx = 1;
    let mut min_diff = f64::MAX;
    for (i, &(opt_val, _)) in NET_OPTIONS.iter().enumerate() {
        if opt_val == 0.0 {
            continue;
        }
        let diff = (val - opt_val).abs();
        if diff < min_diff {
            min_diff = diff;
            best_idx = i;
        }
    }
    best_idx
}

unsafe fn show_menu(hwnd: HWND, ctx: &mut TrayContext) {
    let mut pt = POINT::default();
    let _ = GetCursorPos(&mut pt);
    
    let _ = SetForegroundWindow(hwnd);
    
    let hmenu = CreatePopupMenu().unwrap();
    

    
    let (config, paused) = {
        let s = ctx.state.lock().unwrap();
        (s.config.clone(), s.paused)
    };
    
    let hdelay = CreatePopupMenu().unwrap();
    for (i, &(val, label)) in DELAY_OPTIONS.iter().enumerate() {
        let checked = config.sleep_delay_seconds == val;
        append_menu_item(hdelay, ID_DELAY_START + i, label, checked, true);
    }
    append_submenu(hmenu, hdelay, "スリープ移行時間");
    
    let hcpu = CreatePopupMenu().unwrap();
    let closest_cpu_idx = find_closest_cpu(config.cpu.threshold, config.cpu.enabled);
    for (i, &(_val, label)) in CPU_OPTIONS.iter().enumerate() {
        let checked = closest_cpu_idx == i;
        append_menu_item(hcpu, ID_CPU_START + i, label, checked, true);
    }
    append_submenu(hmenu, hcpu, "スリープ抑止CPU使用率");
    
    let hdisk = CreatePopupMenu().unwrap();
    let closest_disk_idx = find_closest_disk_write(config.disk_write.threshold, config.disk_write.enabled);
    for (i, &(_val, label)) in DISK_WRITE_OPTIONS.iter().enumerate() {
        let checked = closest_disk_idx == i;
        append_menu_item(hdisk, ID_DISK_WRITE_START + i, label, checked, true);
    }
    append_submenu(hmenu, hdisk, "スリープ抑止ディスク書き込み量");
    
    let hnet = CreatePopupMenu().unwrap();
    let closest_net_idx = find_closest_net(config.network.threshold, config.network.enabled);
    for (i, &(_val, label)) in NET_OPTIONS.iter().enumerate() {
        let checked = closest_net_idx == i;
        append_menu_item(hnet, ID_NET_START + i, label, checked, true);
    }
    append_submenu(hmenu, hnet, "スリープ抑止ネットワーク使用量");
    
    let hproc = CreatePopupMenu().unwrap();
    let running_procs = ctx.platform.list_running_processes().unwrap_or_default();
    
    let mut menu_procs = config.watched_processes.clone();
    menu_procs.sort_by_key(|p| p.to_lowercase());
    menu_procs.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
    
    let mut running_clean: Vec<String> = running_procs
        .into_iter()
        .map(|p| {
            std::path::Path::new(&p)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(&p)
                .to_string()
        })
        .collect();
    running_clean.sort_by_key(|p| p.to_lowercase());
    running_clean.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
    
    for rp in running_clean {
        if !menu_procs.iter().any(|wp| wp.to_lowercase() == rp.to_lowercase()) {
            let rp_lower = rp.to_lowercase();
            if rp_lower != "svchost.exe" && rp_lower != "conhost.exe" && rp_lower != "dllhost.exe" {
                menu_procs.push(rp);
            }
        }
    }
    
    if menu_procs.len() > 30 {
        menu_procs.truncate(30);
    }
    
    for (i, name) in menu_procs.iter().enumerate() {
        let checked = config.watched_processes.iter().any(|wp| wp.to_lowercase() == name.to_lowercase());
        append_menu_item(hproc, ID_PROC_START + i, name, checked, true);
    }
    append_submenu(hmenu, hproc, "スリープ抑止プロセス");
    
    append_menu_item(hmenu, ID_SOUND_ENABLED, "サウンド出力によるスリープ抑止", config.sound_enabled, true);
    
    append_separator(hmenu);
    
    append_menu_item(hmenu, ID_DISPLAY_STATE_BY_ICON, "アイコンによる状態表示", config.display_state_by_icon, true);
    append_menu_item(hmenu, ID_WARN_BEFORE_SLEEP, "スリープ前にバルーン表示", config.warn_before_sleep, true);
    append_menu_item(hmenu, ID_WARN_SOUND, "スリープ前に音で警告", config.warn_sound_enabled, true);
    append_menu_item(hmenu, ID_AUTO_START, "ログイン時に自動起動", config.auto_start, true);
    append_menu_item(hmenu, ID_DISPLAY_OFF_ON_SLEEP, "スリープ復帰時画面オフ", config.display_off_on_sleep, true);
    append_menu_item(hmenu, ID_PAUSE, "監視一時停止", paused, true);
    append_menu_item(hmenu, ID_QUIT, "終了", false, true);
    
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
    if cmd_id == 0 {
        let _ = DestroyMenu(hmenu);
        return;
    }
    
    let mut s = ctx.state.lock().unwrap();
    let mut changed = false;
    
    let modifies_config = (cmd_id >= ID_DELAY_START && cmd_id < ID_DELAY_START + DELAY_OPTIONS.len())
        || (cmd_id >= ID_CPU_START && cmd_id < ID_CPU_START + CPU_OPTIONS.len())
        || (cmd_id >= ID_DISK_WRITE_START && cmd_id < ID_DISK_WRITE_START + DISK_WRITE_OPTIONS.len())
        || (cmd_id >= ID_NET_START && cmd_id < ID_NET_START + NET_OPTIONS.len())
        || (cmd_id >= ID_PROC_START && cmd_id < ID_PROC_START + menu_procs.len())
        || cmd_id == ID_SOUND_ENABLED
        || cmd_id == ID_DISPLAY_STATE_BY_ICON
        || cmd_id == ID_WARN_BEFORE_SLEEP
        || cmd_id == ID_WARN_SOUND
        || cmd_id == ID_AUTO_START
        || cmd_id == ID_DISPLAY_OFF_ON_SLEEP;

    if modifies_config {
        let cfg = Arc::make_mut(&mut s.config);
        if cmd_id >= ID_DELAY_START && cmd_id < ID_DELAY_START + DELAY_OPTIONS.len() {
            let idx = cmd_id - ID_DELAY_START;
            cfg.sleep_delay_seconds = DELAY_OPTIONS[idx].0;
            changed = true;
        } else if cmd_id >= ID_CPU_START && cmd_id < ID_CPU_START + CPU_OPTIONS.len() {
            let idx = cmd_id - ID_CPU_START;
            let val = CPU_OPTIONS[idx].0;
            if val == 0.0 {
                cfg.cpu.enabled = false;
            } else {
                cfg.cpu.enabled = true;
                cfg.cpu.threshold = val;
            }
            changed = true;
        } else if cmd_id >= ID_DISK_WRITE_START && cmd_id < ID_DISK_WRITE_START + DISK_WRITE_OPTIONS.len() {
            let idx = cmd_id - ID_DISK_WRITE_START;
            let val = DISK_WRITE_OPTIONS[idx].0;
            if val == 0.0 {
                cfg.disk_write.enabled = false;
            } else {
                cfg.disk_write.enabled = true;
                cfg.disk_write.threshold = val;
            }
            changed = true;
        } else if cmd_id >= ID_NET_START && cmd_id < ID_NET_START + NET_OPTIONS.len() {
            let idx = cmd_id - ID_NET_START;
            let val = NET_OPTIONS[idx].0;
            if val == 0.0 {
                cfg.network.enabled = false;
            } else {
                cfg.network.enabled = true;
                cfg.network.threshold = val;
            }
            changed = true;
        } else if cmd_id >= ID_PROC_START && cmd_id < ID_PROC_START + menu_procs.len() {
            let idx = cmd_id - ID_PROC_START;
            let name = &menu_procs[idx];
            if let Some(pos) = cfg.watched_processes.iter().position(|wp| wp.to_lowercase() == name.to_lowercase()) {
                cfg.watched_processes.remove(pos);
            } else {
                cfg.watched_processes.push(name.clone());
            }
            changed = true;
        } else if cmd_id == ID_SOUND_ENABLED {
            cfg.sound_enabled = !cfg.sound_enabled;
            changed = true;
        } else if cmd_id == ID_DISPLAY_STATE_BY_ICON {
            cfg.display_state_by_icon = !cfg.display_state_by_icon;
            changed = true;
        } else if cmd_id == ID_WARN_BEFORE_SLEEP {
            cfg.warn_before_sleep = !cfg.warn_before_sleep;
            changed = true;
        } else if cmd_id == ID_WARN_SOUND {
            cfg.warn_sound_enabled = !cfg.warn_sound_enabled;
            changed = true;
        } else if cmd_id == ID_AUTO_START {
            cfg.auto_start = !cfg.auto_start;
            let _ = ctx.platform.set_auto_start(cfg.auto_start);
            changed = true;
        } else if cmd_id == ID_DISPLAY_OFF_ON_SLEEP {
            cfg.display_off_on_sleep = !cfg.display_off_on_sleep;
            changed = true;
        }
    } else if cmd_id == ID_PAUSE {
        s.paused = !s.paused;
        changed = true;
    } else if cmd_id == ID_QUIT {
        ctx.running.store(false, Ordering::Relaxed);
        let _ = DestroyWindow(hwnd);
    }
    
    if changed {
        let _ = s.config.save(&Config::config_path());
        let _ = PostMessageW(hwnd, WM_UPDATE_TRAY, WPARAM(0), LPARAM(0));
    }
    
    let _ = DestroyMenu(hmenu);
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let create_struct = &*(lparam.0 as *const CREATESTRUCTW);
            let ctx_ptr = create_struct.lpCreateParams as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ctx_ptr);
            
            let ctx = &mut *(ctx_ptr as *mut TrayContext);
            ctx.hwnd = hwnd;
            
            let mut nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
                uCallbackMessage: WM_TRAYICON,
                hIcon: ctx.hicon_default,
                ..Default::default()
            };
            set_tooltip(&mut nid, "SleepTool Rust");
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
                    (s.current_factor, s.paused, s.config.display_state_by_icon)
                };
                
                let icon_name = factor_icon_name(current_factor, paused);
                let target_icon = match icon_name {
                    "paused" => ctx.hicon_paused,
                    "cpu" => ctx.hicon_cpu,
                    "network" => ctx.hicon_network,
                    "disk" => ctx.hicon_disk,
                    "sound" => ctx.hicon_sound,
                    "process" => ctx.hicon_process,
                    "printer" => ctx.hicon_printer,
                    _ => ctx.hicon_default,
                };
                
                ctx.current_hicon = if display_state_by_icon {
                    target_icon
                } else if paused {
                    ctx.hicon_paused
                } else {
                    ctx.hicon_default
                };
                
                let tooltip = if paused {
                    "SleepTool Rust (一時停止中)"
                } else {
                    match current_factor {
                        Some(InhibitFactor::Printer) => "SleepTool Rust - プリンタ印刷中",
                        Some(InhibitFactor::Process) => "SleepTool Rust - プロセス実行中",
                        Some(InhibitFactor::Sound) => "SleepTool Rust - サウンド出力中",
                        Some(InhibitFactor::Cpu) => "SleepTool Rust - CPU使用中",
                        Some(InhibitFactor::Network) => "SleepTool Rust - ネットワーク使用中",
                        Some(InhibitFactor::DiskRead) => "SleepTool Rust - ディスク読み込み中",
                        Some(InhibitFactor::DiskWrite) => "SleepTool Rust - ディスク書き込み中",
                        Some(InhibitFactor::Input) => "SleepTool Rust - 入力検知中",
                        None => "SleepTool Rust",
                    }
                };
                
                let mut nid = NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: hwnd,
                    uID: 1,
                    uFlags: NIF_ICON | NIF_TIP,
                    hIcon: ctx.current_hicon,
                    ..Default::default()
                };
                set_tooltip(&mut nid, tooltip);
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
                    show_menu(hwnd, ctx);
                }
                return LRESULT(0);
            }
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
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
        
        let hicon_default = load_icon_handle("default")?;
        let hicon_paused = load_icon_handle("paused")?;
        let hicon_cpu = load_icon_handle("cpu")?;
        let hicon_network = load_icon_handle("network")?;
        let hicon_disk = load_icon_handle("disk")?;
        let hicon_sound = load_icon_handle("sound")?;
        let hicon_process = load_icon_handle("process")?;
        let hicon_printer = load_icon_handle("printer")?;
        
        let ctx_box = Box::new(TrayContext {
            state: state.clone(),
            running: running.clone(),
            platform: platform.clone(),
            hwnd: HWND::default(),
            hicon_default,
            hicon_paused,
            hicon_cpu,
            hicon_network,
            hicon_disk,
            hicon_sound,
            hicon_process,
            hicon_printer,
            current_hicon: hicon_default,
        });
        
        let ctx_ptr = Box::into_raw(ctx_box);
        
        let window_name: Vec<u16> = "SleepToolTrayWindow\0".encode_utf16().collect();
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
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
        
        let _ = windows::Win32::UI::WindowsAndMessaging::UnregisterClassW(PCWSTR(class_name.as_ptr()), hinstance);
    }
    
    Ok(())
}
