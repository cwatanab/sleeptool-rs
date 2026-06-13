use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct InputMonitor;

impl InputMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Monitor for InputMonitor {
    fn name(&self) -> &'static str {
        "input"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Input
    }

    fn is_enabled(&self, _config: &Config) -> bool {
        true
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let idle_seconds = platform.last_input_idle_seconds(config.legacy_input)?;
        let threshold = config.sleep_delay_seconds;
        let inhibit = idle_seconds < threshold;
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: idle_seconds as f64,
            threshold: threshold as f64,
        })
    }
}
