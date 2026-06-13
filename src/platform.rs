use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepType {
    Sleep,
    Hibernate,
}

#[derive(Debug, Clone)]
pub struct InputIdleInfo {
    pub idle_seconds: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceSnapshot {
    pub cpu_percent: f64,
    pub network_bytes_per_sec: f64,
    pub disk_read_bytes_per_sec: f64,
    pub disk_write_bytes_per_sec: f64,
}

pub trait Platform: Send + Sync {
    fn last_input_idle_seconds(&self, legacy_input: bool) -> Result<u64>;
    fn query_performance(&self) -> Result<PerformanceSnapshot>;
    fn current_sound_rms(&self) -> Result<f64>;
    fn list_running_processes(&self) -> Result<Vec<String>>;
    fn has_print_jobs(&self, printer_names: &[String]) -> Result<bool>;
    fn list_printers(&self) -> Result<Vec<String>>;
    fn set_auto_start(&self, enable: bool) -> Result<()>;
    fn is_auto_start_enabled(&self) -> Result<bool>;
    fn suspend(&self, sleep_type: SleepType, force: bool) -> Result<()>;
    fn turn_display_off(&self) -> Result<()>;
    fn show_sleep_warning(&self, seconds: u64, play_sound: bool) -> Result<bool>;
}
