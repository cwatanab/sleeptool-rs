mod audio;
mod notify;
mod pdh;
mod power;
mod registry;
mod util;

use std::collections::HashSet;
use std::sync::atomic::AtomicIsize;
use std::sync::Mutex;
use std::time::Instant;

use windows::Win32::Foundation::POINT;
use windows::Win32::System::Performance::PdhCloseQuery;
use windows::Win32::System::ProcessStatus::EnumProcesses;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::error::Result;
use crate::platform::{
    AudioProbe, InputProbe, Notifier, PerformanceProbe, PerformanceSnapshot, Platform,
    PowerControl, ProcessProbe, SleepType, StartupControl,
};

use crate::platform_win32::util::{check_bool, win_result};

struct LegacyInputState {
    last_cursor_pos: POINT,
    last_input_time: Instant,
}

struct PerfCache {
    last_collect: Option<Instant>,
    smoothed: PerformanceSnapshot,
}

struct ProcessCache {
    last_update: Option<Instant>,
    names: HashSet<String>,
}

pub struct WindowsPlatform {
    query: isize,
    cpu_counter: Option<isize>,
    network_counter: Option<isize>,
    disk_read_counter: Option<isize>,
    disk_write_counter: Option<isize>,
    legacy_input_state: Mutex<LegacyInputState>,
    perf_cache: Mutex<PerfCache>,
    process_cache: Mutex<ProcessCache>,
    pub tray_hwnd: AtomicIsize,
    smoothing_alpha: f64,
}

impl WindowsPlatform {
    pub fn new(smoothing_alpha: f64) -> Result<Self> {
        unsafe {
            let query = pdh::open_query()?;
            let counters = pdh::add_standard_counters(query)?;
            let _ = pdh::collect(query);

            let mut last_cursor_pos = POINT::default();
            let _ = GetCursorPos(&mut last_cursor_pos);

            Ok(Self {
                query,
                cpu_counter: counters.cpu,
                network_counter: counters.network,
                disk_read_counter: counters.disk_read,
                disk_write_counter: counters.disk_write,
                legacy_input_state: Mutex::new(LegacyInputState {
                    last_cursor_pos,
                    last_input_time: Instant::now(),
                }),
                perf_cache: Mutex::new(PerfCache {
                    last_collect: None,
                    smoothed: PerformanceSnapshot::default(),
                }),
                process_cache: Mutex::new(ProcessCache { last_update: None, names: HashSet::with_capacity(64) }),
                tray_hwnd: AtomicIsize::new(0),
                smoothing_alpha,
            })
        }
    }

    pub fn reset_smoothing(&self) {
        self.perf_cache.lock().unwrap().smoothed = PerformanceSnapshot::default();
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe { let _ = PdhCloseQuery(self.query); }
    }
}

impl Platform for WindowsPlatform {}

impl PerformanceProbe for WindowsPlatform {
    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        let mut cache = self.perf_cache.lock().unwrap();
        if let Some(last_time) = cache.last_collect {
            if last_time.elapsed() < std::time::Duration::from_millis(500) {
                return Ok(cache.smoothed);
            }
        }

        unsafe {
            let _ = pdh::collect(self.query);
            let raw = PerformanceSnapshot {
                cpu_percent: pdh::try_get(self.cpu_counter),
                network_bytes_per_sec: pdh::try_get(self.network_counter),
                disk_read_bytes_per_sec: pdh::try_get(self.disk_read_counter),
                disk_write_bytes_per_sec: pdh::try_get(self.disk_write_counter),
            };

            let snapshot = PerformanceSnapshot {
                cpu_percent: pdh::smooth(cache.smoothed.cpu_percent, raw.cpu_percent, self.smoothing_alpha),
                network_bytes_per_sec: pdh::smooth(cache.smoothed.network_bytes_per_sec, raw.network_bytes_per_sec, self.smoothing_alpha),
                disk_read_bytes_per_sec: pdh::smooth(cache.smoothed.disk_read_bytes_per_sec, raw.disk_read_bytes_per_sec, self.smoothing_alpha),
                disk_write_bytes_per_sec: pdh::smooth(cache.smoothed.disk_write_bytes_per_sec, raw.disk_write_bytes_per_sec, self.smoothing_alpha),
            };
            cache.smoothed = snapshot;
            cache.last_collect = Some(Instant::now());
            Ok(snapshot)
        }
    }
}

impl InputProbe for WindowsPlatform {
    fn last_input_idle_seconds(&self, legacy_input: bool) -> Result<u64> {
        unsafe {
            if legacy_input {
                let mut current_pos = POINT::default();
                if GetCursorPos(&mut current_pos).is_err() {
                    return Ok(0);
                }
                let mut key_pressed = false;
                for vk in 1u8..=255 {
                    if (GetAsyncKeyState(vk as i32) & 0x8000u16 as i16) != 0 {
                        key_pressed = true;
                        break;
                    }
                }
                let mut state = self.legacy_input_state.lock().unwrap();
                if current_pos.x != state.last_cursor_pos.x
                    || current_pos.y != state.last_cursor_pos.y
                    || key_pressed
                {
                    state.last_cursor_pos = current_pos;
                    state.last_input_time = Instant::now();
                    Ok(0)
                } else {
                    Ok(state.last_input_time.elapsed().as_secs())
                }
            } else {
                let mut info = LASTINPUTINFO {
                    cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                    dwTime: 0,
                };
                check_bool(GetLastInputInfo(&mut info))?;
                let tick = windows::Win32::System::SystemInformation::GetTickCount();
                Ok(((tick - info.dwTime) / 1000) as u64)
            }
        }
    }
}

impl AudioProbe for WindowsPlatform {
    fn current_sound_rms(&self) -> Result<f64> { audio::current_peak() }
}

impl ProcessProbe for WindowsPlatform {
    fn list_running_processes(&self) -> Result<Vec<String>> {
        let mut cache = self.process_cache.lock().unwrap();
        if cache.last_update.map_or(false, |t| t.elapsed() < std::time::Duration::from_secs(5)) {
            return Ok(cache.names.iter().cloned().collect());
        }

        cache.names.clear();
        cache.names.reserve(128);
        unsafe {
            let mut pids = [0u32; 2048];
            let mut needed = 0u32;
            win_result(EnumProcesses(pids.as_mut_ptr(), std::mem::size_of_val(&pids) as u32, &mut needed))?;
            let count = (needed as usize / std::mem::size_of::<u32>()).min(2048);
            for &pid in &pids[..count] {
                if pid == 0 { continue; }
                let handle = OpenProcess(
                    windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION
                        | windows::Win32::System::Threading::PROCESS_VM_READ,
                    false, pid,
                );
                if let Ok(handle) = handle {
                    let mut buf = [0u16; 260];
                    let len = windows::Win32::System::ProcessStatus::GetProcessImageFileNameW(handle, &mut buf);
                    if len > 0 {
                        if let Some(name) = String::from_utf16_lossy(&buf[..len as usize]).rsplit('\\').next() {
                            cache.names.insert(name.to_ascii_lowercase());
                        }
                    }
                    let _ = windows::Win32::Foundation::CloseHandle(handle);
                }
            }
        }
        cache.last_update = Some(Instant::now());
        Ok(cache.names.iter().cloned().collect())
    }
}

impl PowerControl for WindowsPlatform {
    fn suspend(&self, sleep_type: SleepType, force: bool) -> Result<()> { power::suspend(sleep_type, force) }
    fn turn_display_off(&self) -> Result<()> { power::turn_display_off() }
}

impl Notifier for WindowsPlatform {
    fn show_sleep_warning(&self, seconds: u64, _play_sound: bool) -> Result<bool> { notify::show_sleep_warning(self, seconds) }
}

impl StartupControl for WindowsPlatform {
    fn set_auto_start(&self, enable: bool) -> Result<()> { registry::set_auto_start(enable) }
    fn is_auto_start_enabled(&self) -> Result<bool> { registry::is_auto_start_enabled() }
}

pub fn optimize_memory() {
    unsafe {
        let handle = windows::Win32::System::Threading::GetCurrentProcess();
        let _ = windows::Win32::System::ProcessStatus::EmptyWorkingSet(handle);
    }
}
