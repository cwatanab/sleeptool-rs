//! Windows 専用 `Platform` 実装。
//!
//! サブモジュール:
//! - `util`: Win32 API エラーチェック
//! - `pdh`: PDH 性能クエリ
//! - `audio`: COM 経由の音声メーター
//! - `registry`: 自動起動
//! - `power`: スリープ / ディスプレイオフ
//! - `notify`: トレイバルーン

mod audio;
mod notify;
mod pdh;
mod power;
mod registry;
mod util;

use std::sync::atomic::AtomicIsize;
use std::sync::Mutex;
use std::time::Instant;

use windows::Win32::Foundation::POINT;
use windows::Win32::System::Performance::PdhCloseQuery;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::ProcessStatus::EnumProcesses;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::error::Result;
use crate::platform::{
    AudioProbe, InputProbe, Notifier, PerformanceProbe, PerformanceSnapshot, Platform,
    PowerControl, ProcessProbe, SleepType, StartupControl,
};

use crate::platform_win32::util::{check_bool, win_result};

/// `GetLastInputInfo` 用レガシーモード状態。
struct LegacyInputState {
    last_cursor_pos: POINT,
    last_input_time: Instant,
}

/// 性能スナップショットのキャッシュと EMA 平滑化。
struct PerfCache {
    last_collect: Option<Instant>,
    last_snapshot: PerformanceSnapshot,
    smoothed: PerformanceSnapshot,
}

pub struct WindowsPlatform {
    query: isize,
    cpu_counter: Option<isize>,
    network_counter: Option<isize>,
    disk_read_counter: Option<isize>,
    disk_write_counter: Option<isize>,
    legacy_input_state: Mutex<LegacyInputState>,
    perf_cache: Mutex<PerfCache>,
    pub tray_hwnd: AtomicIsize,
}

impl WindowsPlatform {
    pub fn new() -> Result<Self> {
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
                    last_snapshot: PerformanceSnapshot::default(),
                    smoothed: PerformanceSnapshot::default(),
                }),
                tray_hwnd: AtomicIsize::new(0),
            })
        }
    }

    pub fn reset_smoothing(&self) {
        let mut cache = self.perf_cache.lock().unwrap();
        cache.smoothed = PerformanceSnapshot::default();
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe {
            let _ = PdhCloseQuery(self.query);
        }
    }
}

impl Platform for WindowsPlatform {}

impl PerformanceProbe for WindowsPlatform {
    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        let mut cache = self.perf_cache.lock().unwrap();
        if let Some(last_time) = cache.last_collect {
            if last_time.elapsed() < std::time::Duration::from_millis(500) {
                return Ok(cache.last_snapshot);
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
                cpu_percent: pdh::smooth(cache.smoothed.cpu_percent, raw.cpu_percent),
                network_bytes_per_sec: pdh::smooth(
                    cache.smoothed.network_bytes_per_sec,
                    raw.network_bytes_per_sec,
                ),
                disk_read_bytes_per_sec: pdh::smooth(
                    cache.smoothed.disk_read_bytes_per_sec,
                    raw.disk_read_bytes_per_sec,
                ),
                disk_write_bytes_per_sec: pdh::smooth(
                    cache.smoothed.disk_write_bytes_per_sec,
                    raw.disk_write_bytes_per_sec,
                ),
            };
            cache.smoothed = snapshot;
            cache.last_snapshot = snapshot;
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
                if GetCursorPos(&mut current_pos).is_ok() {
                    let mut key_pressed = false;
                    for vk in 1..=255 {
                        let state = GetAsyncKeyState(vk);
                        if (state & 0x8000u16 as i16) != 0 {
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
                        return Ok(0);
                    } else {
                        return Ok(state.last_input_time.elapsed().as_secs());
                    }
                }
                Ok(0)
            } else {
                let mut info = LASTINPUTINFO {
                    cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                    dwTime: 0,
                };
                check_bool(GetLastInputInfo(&mut info))?;
                let tick = windows::Win32::System::SystemInformation::GetTickCount();
                let idle_ms = tick - info.dwTime;
                Ok((idle_ms / 1000) as u64)
            }
        }
    }
}

impl AudioProbe for WindowsPlatform {
    fn current_sound_rms(&self) -> Result<f64> {
        audio::current_peak()
    }
}

impl ProcessProbe for WindowsPlatform {
    fn list_running_processes(&self) -> Result<Vec<String>> {
        unsafe {
            let mut pids = [0u32; 2048];
            let mut needed = 0u32;
            win_result(EnumProcesses(pids.as_mut_ptr(), std::mem::size_of_val(&pids) as u32, &mut needed))?;
            let count = needed as usize / std::mem::size_of::<u32>();

            let mut names = Vec::new();
            for &pid in &pids[..count] {
                if pid == 0 {
                    continue;
                }
                let handle = OpenProcess(
                    windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION
                        | windows::Win32::System::Threading::PROCESS_VM_READ,
                    false,
                    pid,
                );
                if let Ok(handle) = handle {
                    let mut buf = [0u16; 260];
                    let len = windows::Win32::System::ProcessStatus::GetProcessImageFileNameW(
                        handle,
                        &mut buf,
                    );
                    if len > 0 {
                        let path = String::from_utf16_lossy(&buf[..len as usize]);
                        if let Some(name) = path.rsplit('\\').next() {
                            names.push(name.to_lowercase());
                        }
                    }
                    let _ = windows::Win32::Foundation::CloseHandle(handle);
                }
            }
            Ok(names)
        }
    }
}

impl PowerControl for WindowsPlatform {
    fn suspend(&self, sleep_type: SleepType, force: bool) -> Result<()> {
        power::suspend(sleep_type, force)
    }

    fn turn_display_off(&self) -> Result<()> {
        power::turn_display_off()
    }
}

impl Notifier for WindowsPlatform {
    fn show_sleep_warning(&self, seconds: u64, _play_sound: bool) -> Result<bool> {
        notify::show_sleep_warning(self, seconds)
    }
}

impl StartupControl for WindowsPlatform {
    fn set_auto_start(&self, enable: bool) -> Result<()> {
        registry::set_auto_start(enable)
    }

    fn is_auto_start_enabled(&self) -> Result<bool> {
        registry::is_auto_start_enabled()
    }
}
