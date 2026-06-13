use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct NetworkMonitor {
    last_active: Option<std::time::Instant>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self { last_active: None }
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

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let perf = platform.query_performance()?;
        let cfg = &config.network;
        let active = perf.network_bytes_per_sec >= cfg.threshold;

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
            value: perf.network_bytes_per_sec,
            threshold: cfg.threshold,
        })
    }
}
