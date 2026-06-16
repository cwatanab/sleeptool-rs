use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{AudioProbe, PerformanceSnapshot, Platform};

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
        config.sound.enabled
    }

    fn sample(&mut self, _config: &Config, platform: &dyn Platform, _perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let rms = AudioProbe::current_sound_rms(platform)?;
        let threshold = 0.01;
        Ok(MonitorState {
            inhibit: rms >= threshold,
            factor: self.default_factor(),
            value: rms,
            threshold,
        })
    }
}
