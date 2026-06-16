use crate::config::Config;
use crate::error::Result;
use crate::monitors::threshold::{evaluate, ThresholdState};
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{PerformanceSnapshot, Platform};

pub struct NetworkMonitor {
    state: ThresholdState,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self { state: ThresholdState::default() }
    }
}

impl Monitor for NetworkMonitor {
    fn name(&self) -> &'static str {
        "network"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Network
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.network.enabled
    }

    fn sample(&mut self, config: &Config, _platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let cfg = &config.network;
        let inhibit = evaluate(&mut self.state, perf.network_bytes_per_sec, cfg.threshold, cfg.delay_seconds);
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.network_bytes_per_sec,
            threshold: cfg.threshold,
        })
    }
}
