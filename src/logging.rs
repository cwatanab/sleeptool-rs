use crate::config::LogLevel;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

static LOG_WRITER: Mutex<Option<BufWriter<std::fs::File>>> = Mutex::new(None);
static LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Off);

pub fn init_logging(log_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(log_dir)?;
    let log_path = log_dir.join("sleeptool.log");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_path)?;
    *LOG_WRITER.lock().unwrap() = Some(BufWriter::new(file));
    Ok(())
}

pub fn set_log_level(level: LogLevel) {
    *LOG_LEVEL.lock().unwrap() = level;
}

fn get_timestamp() -> String {
    use windows::Win32::System::SystemInformation::GetLocalTime;
    unsafe {
        let st = GetLocalTime();
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            st.wYear,
            st.wMonth,
            st.wDay,
            st.wHour,
            st.wMinute,
            st.wSecond,
            st.wMilliseconds
        )
    }
}

fn level_str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => "WARN",
        LogLevel::Info => "INFO",
        LogLevel::Debug => "DEBUG",
        LogLevel::Off => "OFF",
    }
}

pub fn log_message(level: LogLevel, msg: &str) {
    if level > *LOG_LEVEL.lock().unwrap() {
        return;
    }

    let timestamp = get_timestamp();
    if let Ok(mut guard) = LOG_WRITER.lock() {
        if let Some(ref mut writer) = *guard {
            let _ = writeln!(writer, "[{}] {} - {}", timestamp, level_str(level), msg);
            let _ = writer.flush();
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logging::log_message($crate::config::LogLevel::Info, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logging::log_message($crate::config::LogLevel::Error, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logging::log_message($crate::config::LogLevel::Debug, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logging::log_message($crate::config::LogLevel::Warn, &format!($($arg)*));
    };
}
