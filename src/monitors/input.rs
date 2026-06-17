use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{InputProbe, PerformanceSnapshot, Platform};

pub struct InputMonitor;

impl InputMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Monitor for InputMonitor {
    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Input
    }

    fn is_enabled(&self, _config: &Config) -> bool {
        true
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform, _perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let idle_seconds = InputProbe::last_input_idle_seconds(platform, config.general.legacy_input)?;
        let threshold = config.sleep.delay_seconds;
        let inhibit = idle_seconds < threshold;
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: idle_seconds as f64,
            threshold: threshold as f64,
        })
    }
}
