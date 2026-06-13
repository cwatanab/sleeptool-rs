use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct CpuMonitor {
    last_active: Option<std::time::Instant>,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self { last_active: None }
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

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let perf = platform.query_performance()?;
        let cfg = &config.cpu;
        let active = perf.cpu_percent >= cfg.threshold;

        let now = std::time::Instant::now();
        if active {
            self.last_active = Some(now);
        }

        let inhibit = if active {
            true
        } else if let Some(last) = self.last_active {
            now.duration_since(last).as_secs() < cfg.delay_seconds
        } else {
            false
        };

        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.cpu_percent,
            threshold: cfg.threshold,
        })
    }
}
