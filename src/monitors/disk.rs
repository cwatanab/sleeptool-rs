use crate::config::Config;
use crate::error::Result;
use crate::monitors::threshold::{evaluate, ThresholdState};
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{PerformanceSnapshot, Platform};

pub struct DiskWriteMonitor {
    state: ThresholdState,
}
pub struct DiskReadMonitor {
    state: ThresholdState,
}

impl DiskWriteMonitor {
    pub fn new() -> Self {
        Self { state: ThresholdState::default() }
    }
}

impl Monitor for DiskWriteMonitor {
    fn name(&self) -> &'static str {
        "disk_write"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::DiskWrite
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.disk.write_enabled
    }

    fn sample(&mut self, config: &Config, _platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let inhibit = evaluate(
            &mut self.state,
            perf.disk_write_bytes_per_sec,
            config.disk.write_threshold,
            config.disk.write_delay_seconds,
        );
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.disk_write_bytes_per_sec,
            threshold: config.disk.write_threshold,
        })
    }
}

impl DiskReadMonitor {
    pub fn new() -> Self {
        Self { state: ThresholdState::default() }
    }
}

impl Monitor for DiskReadMonitor {
    fn name(&self) -> &'static str {
        "disk_read"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::DiskRead
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.disk.read_enabled
    }

    fn sample(&mut self, config: &Config, _platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let inhibit = evaluate(
            &mut self.state,
            perf.disk_read_bytes_per_sec,
            config.disk.read_threshold,
            config.disk.read_delay_seconds,
        );
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.disk_read_bytes_per_sec,
            threshold: config.disk.read_threshold,
        })
    }
}
