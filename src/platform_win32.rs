use crate::error::{Result, SleepToolError};
use crate::platform::{Platform, PerformanceSnapshot, SleepType};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, BOOLEAN, HWND, LPARAM, WPARAM, POINT};
use windows::Win32::System::Performance::{
    PdhAddCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_DOUBLE, PDH_FMT_COUNTERVALUE,
};
use windows::Win32::System::Power::SetSuspendState;
use windows::Win32::Media::Audio::{
    eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Graphics::Printing::{EnumJobsW, OpenPrinterW};
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ,
};
use windows::Win32::System::ProcessStatus::{EnumProcesses, GetProcessImageFileNameW};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO, GetAsyncKeyState};
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND, GetCursorPos
};
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};

struct LegacyInputState {
    last_cursor_pos: POINT,
    last_input_time: std::time::Instant,
}

pub struct WindowsPlatform {
    query: isize,
    cpu_counter: Option<isize>,
    network_counter: Option<isize>,
    disk_read_counter: Option<isize>,
    disk_write_counter: Option<isize>,
    legacy_input_state: std::sync::Mutex<LegacyInputState>,
    perf_cache: std::sync::Mutex<Option<(std::time::Instant, PerformanceSnapshot)>>,
}

impl WindowsPlatform {
    pub fn new() -> Result<Self> {
        unsafe {
            let mut query = 0isize;
            check_pdh(PdhOpenQueryW(None, 0, &mut query))?;

            let cpu_counter = add_counter(query, r"\Processor(_Total)\% Processor Time").ok();
            let network_counter = add_counter(query, r"\Network Interface(*)\Bytes Total/sec").ok();
            let disk_read_counter = add_counter(query, r"\PhysicalDisk(_Total)\Disk Read Bytes/sec").ok();
            let disk_write_counter = add_counter(query, r"\PhysicalDisk(_Total)\Disk Write Bytes/sec").ok();

            // First collect is needed to establish baseline
            let _ = PdhCollectQueryData(query);

            let mut last_cursor_pos = POINT::default();
            let _ = GetCursorPos(&mut last_cursor_pos);

            Ok(Self {
                query,
                cpu_counter,
                network_counter,
                disk_read_counter,
                disk_write_counter,
                legacy_input_state: std::sync::Mutex::new(LegacyInputState {
                    last_cursor_pos,
                    last_input_time: std::time::Instant::now(),
                }),
                perf_cache: std::sync::Mutex::new(None),
            })
        }
    }
}

fn check_pdh(status: u32) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform(format!("PDH error: 0x{:08X}", status)))
    }
}

fn check_bool(result: BOOL) -> Result<()> {
    if result.as_bool() {
        Ok(())
    } else {
        Err(SleepToolError::Platform("Windows API returned false".to_string()))
    }
}

fn check_boolean(result: BOOLEAN) -> Result<()> {
    if result.0 != 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform("Windows API returned false".to_string()))
    }
}

fn win_result<T>(result: windows::core::Result<T>) -> Result<T> {
    result.map_err(|e| SleepToolError::Platform(e.to_string()))
}

// capture_loopback_rms deleted in favor of direct audio meter peak check

fn check_win32(status: windows::Win32::Foundation::WIN32_ERROR) -> Result<()> {
    if status.0 == 0 {
        Ok(())
    } else {
        Err(SleepToolError::Platform(format!("Windows error: 0x{:08X}", status.0)))
    }
}

fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

unsafe fn add_counter(query: isize, path: &str) -> Result<isize> {
    let wpath = to_wstring(path);
    let mut counter = 0isize;
    check_pdh(PdhAddCounterW(query, PCWSTR(wpath.as_ptr()), 0, &mut counter))?;
    Ok(counter)
}

unsafe fn get_counter_value(counter: isize) -> Result<f64> {
    let mut value = PDH_FMT_COUNTERVALUE::default();
    check_pdh(PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value))?;
    Ok(value.Anonymous.doubleValue)
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe {
            let _ = PdhCloseQuery(self.query);
        }
    }
}

impl Platform for WindowsPlatform {
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
                        state.last_input_time = std::time::Instant::now();
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
                let tick = GetTickCount();
                let idle_ms = tick - info.dwTime;
                Ok((idle_ms / 1000) as u64)
            }
        }
    }

    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        let mut cache = self.perf_cache.lock().unwrap();
        if let Some((last_time, snapshot)) = &*cache {
            if last_time.elapsed() < std::time::Duration::from_millis(500) {
                return Ok(snapshot.clone());
            }
        }

        unsafe {
            let _ = PdhCollectQueryData(self.query);
            let snapshot = PerformanceSnapshot {
                cpu_percent: self.cpu_counter.and_then(|c| get_counter_value(c).ok()).unwrap_or(0.0),
                network_bytes_per_sec: self.network_counter.and_then(|c| get_counter_value(c).ok()).unwrap_or(0.0),
                disk_read_bytes_per_sec: self.disk_read_counter.and_then(|c| get_counter_value(c).ok()).unwrap_or(0.0),
                disk_write_bytes_per_sec: self.disk_write_counter.and_then(|c| get_counter_value(c).ok()).unwrap_or(0.0),
            };
            *cache = Some((std::time::Instant::now(), snapshot.clone()));
            Ok(snapshot)
        }
    }

    fn current_sound_rms(&self) -> Result<f64> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            let result = (|| -> windows::core::Result<f64> {
                let enumerator: IMMDeviceEnumerator = CoCreateInstance(
                    &MMDeviceEnumerator,
                    None,
                    CLSCTX_ALL,
                )?;
                let device = enumerator.GetDefaultAudioEndpoint(
                    eRender,
                    windows::Win32::Media::Audio::eConsole,
                )?;
                let meter: IAudioMeterInformation = device.Activate(
                    CLSCTX_ALL,
                    None,
                )?;
                let peak = meter.GetPeakValue()?;
                Ok(peak as f64)
            })();
            windows::Win32::System::Com::CoUninitialize();
            
            match result {
                Ok(val) => Ok(val),
                Err(_) => Ok(0.0),
            }
        }
    }

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
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    false,
                    pid,
                );
                if let Ok(handle) = handle {
                    let mut buf = [0u16; 260];
                    let len = GetProcessImageFileNameW(handle, &mut buf);
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

    fn has_print_jobs(&self, printer_names: &[String]) -> Result<bool> {
        unsafe {
            if printer_names.is_empty() {
                return Ok(false);
            }

            for name in printer_names {
                let wname = to_wstring(name);
                let mut handle = windows::Win32::Foundation::HANDLE::default();
                if OpenPrinterW(PCWSTR(wname.as_ptr()), &mut handle, None).is_ok() {
                    let mut needed = 0u32;
                    let mut returned = 0u32;
                    let _ = EnumJobsW(handle, 0, 1, 1, None, &mut needed, &mut returned);
                    let _ = windows::Win32::Graphics::Printing::ClosePrinter(handle);
                    if returned > 0 {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }
    }

    fn list_printers(&self) -> Result<Vec<String>> {
        unsafe {
            use windows::Win32::Graphics::Printing::{EnumPrintersW, PRINTER_ENUM_LOCAL, PRINTER_ENUM_CONNECTIONS, PRINTER_INFO_4W};
            let flags = PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS;
            let mut needed = 0;
            let mut returned = 0;
            let _ = EnumPrintersW(flags, PCWSTR(std::ptr::null()), 4, None, &mut needed, &mut returned);

            if needed == 0 {
                return Ok(vec![]);
            }

            let mut buffer = vec![0u8; needed as usize];
            win_result(EnumPrintersW(
                flags,
                PCWSTR(std::ptr::null()),
                4,
                Some(&mut buffer),
                &mut needed,
                &mut returned,
            ))?;

            let infos = std::slice::from_raw_parts(
                buffer.as_ptr() as *const PRINTER_INFO_4W,
                returned as usize,
            );

            let mut printers = Vec::new();
            for info in infos {
                if !info.pPrinterName.0.is_null() {
                    let name = info.pPrinterName.to_string().unwrap_or_default();
                    if !name.is_empty() {
                        printers.push(name);
                    }
                }
            }
            Ok(printers)
        }
    }

    fn set_auto_start(&self, enable: bool) -> Result<()> {
        unsafe {
            let subkey = to_wstring(r"Software\Microsoft\Windows\CurrentVersion\Run");
            let value_name = to_wstring("SleepToolRust");

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

    fn is_auto_start_enabled(&self) -> Result<bool> {
        unsafe {
            let subkey = to_wstring(r"Software\Microsoft\Windows\CurrentVersion\Run");
            let value_name = to_wstring("SleepToolRust");

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

    fn suspend(&self, sleep_type: SleepType, force: bool) -> Result<()> {
        unsafe {
            let hibernate = sleep_type == SleepType::Hibernate;
            let result = SetSuspendState(
                BOOLEAN(if hibernate { 1 } else { 0 }),
                BOOLEAN(if force { 1 } else { 0 }),
                BOOLEAN(0),
            );
            check_boolean(result)?;
            Ok(())
        }
    }

    fn turn_display_off(&self) -> Result<()> {
        unsafe {
            SendMessageW(
                HWND(std::ptr::null_mut()),
                WM_SYSCOMMAND,
                WPARAM(SC_MONITORPOWER as usize),
                LPARAM(2),
            );
            Ok(())
        }
    }

    fn show_sleep_warning(&self, seconds: u64, play_sound: bool) -> Result<bool> {
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{
                FindWindowW, PostMessageW, MB_OKCANCEL, MB_ICONWARNING, WM_CLOSE, IDCANCEL
            };

            let (tx, rx) = std::sync::mpsc::channel();
            let title = to_wstring("SleepTool");
            let message = to_wstring(&format!(
                "スリープ移行の{}秒前です。\nキャンセルするとスリープを中止します。",
                seconds
            ));

            let utype = if play_sound {
                MB_OKCANCEL | MB_ICONWARNING
            } else {
                MB_OKCANCEL
            };

            let title_clone = title.clone();
            std::thread::spawn(move || {
                let user32_name = to_wstring("user32.dll");
                let user32_module = GetModuleHandleW(PCWSTR(user32_name.as_ptr())).unwrap_or_default();
                let proc_addr = if !user32_module.is_invalid() {
                    GetProcAddress(user32_module, windows::core::PCSTR(b"MessageBoxTimeoutW\0".as_ptr()))
                } else {
                    None
                };

                type MessageBoxTimeoutWFn = unsafe extern "system" fn(
                    HWND,
                    PCWSTR,
                    PCWSTR,
                    u32,
                    u16,
                    u32,
                ) -> i32;

                let res = if let Some(f) = proc_addr {
                    let func: MessageBoxTimeoutWFn = std::mem::transmute(f);
                    func(
                        HWND(std::ptr::null_mut()),
                        PCWSTR(message.as_ptr()),
                        PCWSTR(title_clone.as_ptr()),
                        utype.0,
                        0,
                        (seconds * 1000) as u32,
                    )
                } else {
                    use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
                    MessageBoxW(
                        HWND(std::ptr::null_mut()),
                        PCWSTR(message.as_ptr()),
                        PCWSTR(title_clone.as_ptr()),
                        utype,
                    ).0
                };
                let _ = tx.send(res);
            });

            // Loop and check if user input is detected.
            // Check every 100ms.
            let start_idle = self.last_input_idle_seconds(false).unwrap_or(0);
            let check_interval = std::time::Duration::from_millis(100);
            let total_ticks = seconds * 10;

            for _ in 0..total_ticks {
                if let Ok(res) = rx.try_recv() {
                    return Ok(res == IDCANCEL.0);
                }

                let current_idle = self.last_input_idle_seconds(false).unwrap_or(0);
                if current_idle < start_idle || current_idle == 0 {
                    if let Ok(hwnd) = FindWindowW(None, PCWSTR(title.as_ptr())) {
                        if !hwnd.0.is_null() {
                            let _ = PostMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
                        }
                    }
                    return Ok(true); // User active, cancel sleep
                }

                std::thread::sleep(check_interval);
            }

            let res = rx.recv().unwrap_or(0);
            Ok(res == IDCANCEL.0)
        }
    }
}
