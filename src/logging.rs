use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn init_logging(log_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(log_dir)?;
    let log_path = log_dir.join("sleeptool.log");
    // Try to open it to make sure it's writable
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&log_path)?;
    let mut guard = LOG_FILE.lock().unwrap();
    *guard = Some(log_path);
    Ok(())
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

pub fn log_message(level: &str, msg: &str) {
    let log_path = {
        let guard = LOG_FILE.lock().unwrap();
        guard.clone()
    };
    if let Some(path) = log_path {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)
        {
            let timestamp = get_timestamp();
            let _ = writeln!(file, "[{}] {} - {}", timestamp, level, msg);
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logging::log_message("INFO", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logging::log_message("ERROR", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logging::log_message("DEBUG", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logging::log_message("WARN", &format!($($arg)*));
    };
}
