use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::monitors::threshold::ThresholdMonitor;
use crate::platform::{PerformanceSnapshot, Platform};

pub struct DiskWriteMonitor(ThresholdMonitor);
pub struct DiskReadMonitor(ThresholdMonitor);

impl DiskWriteMonitor {
    pub fn new() -> Self {
        Self(ThresholdMonitor::new(
            InhibitFactor::DiskWrite,
            |p| p.disk_write_bytes_per_sec,
            |c| c.disk.write_enabled,
            |c| c.disk.write_threshold,
            |c| c.disk.write_delay_seconds,
        ))
    }
}

impl DiskReadMonitor {
    pub fn new() -> Self {
        Self(ThresholdMonitor::new(
            InhibitFactor::DiskRead,
            |p| p.disk_read_bytes_per_sec,
            |c| c.disk.read_enabled,
            |c| c.disk.read_threshold,
            |c| c.disk.read_delay_seconds,
        ))
    }
}

impl Monitor for DiskWriteMonitor {
    fn default_factor(&self) -> InhibitFactor { self.0.factor() }
    fn is_enabled(&self, config: &Config) -> bool { self.0.is_enabled(config) }
    fn sample(&mut self, config: &Config, platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        self.0.sample(config, platform, perf)
    }
}

impl Monitor for DiskReadMonitor {
    fn default_factor(&self) -> InhibitFactor { self.0.factor() }
    fn is_enabled(&self, config: &Config) -> bool { self.0.is_enabled(config) }
    fn sample(&mut self, config: &Config, platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        self.0.sample(config, platform, perf)
    }
}
