use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::monitors::threshold::ThresholdMonitor;
use crate::platform::{PerformanceSnapshot, Platform};

pub struct CpuMonitor(ThresholdMonitor);

impl CpuMonitor {
    pub fn new() -> Self {
        Self(ThresholdMonitor::new(
            InhibitFactor::Cpu,
            |p| p.cpu_percent,
            |c| c.cpu.enabled,
            |c| c.cpu.threshold,
            |c| c.cpu.delay_seconds,
        ))
    }
}

impl Monitor for CpuMonitor {
    fn default_factor(&self) -> InhibitFactor { self.0.factor() }
    fn is_enabled(&self, config: &Config) -> bool { self.0.is_enabled(config) }
    fn sample(&mut self, config: &Config, platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        self.0.sample(config, platform, perf)
    }
}
