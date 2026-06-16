use crate::config::Config;
use crate::error::Result;
use crate::monitors::threshold::{evaluate, ThresholdState};
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{PerformanceSnapshot, Platform};

pub struct CpuMonitor {
    state: ThresholdState,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self { state: ThresholdState::default() }
    }
}

impl Monitor for CpuMonitor {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Cpu
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.cpu.enabled
    }

    fn sample(&mut self, config: &Config, _platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let cfg = &config.cpu;
        let inhibit = evaluate(&mut self.state, perf.cpu_percent, cfg.threshold, cfg.delay_seconds);
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.cpu_percent,
            threshold: cfg.threshold,
        })
    }
}
