//! プラットフォーム抽象で使われる値オブジェクト。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepType {
    Sleep,
    Hibernate,
}

#[derive(Debug, Clone)]
pub struct InputIdleInfo {
    pub idle_seconds: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PerformanceSnapshot {
    pub cpu_percent: f64,
    pub network_bytes_per_sec: f64,
    pub disk_read_bytes_per_sec: f64,
    pub disk_write_bytes_per_sec: f64,
}
