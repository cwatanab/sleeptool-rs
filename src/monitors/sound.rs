use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct SoundMonitor;

impl SoundMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Monitor for SoundMonitor {
    fn name(&self) -> &'static str {
        "sound"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Sound
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.sound_enabled
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let rms = if self.is_enabled(config) {
            platform.current_sound_rms()?
        } else {
            0.0
        };
        let threshold = 0.01;
        Ok(MonitorState {
            inhibit: rms >= threshold,
            factor: self.default_factor(),
            value: rms,
            threshold,
        })
    }
}
