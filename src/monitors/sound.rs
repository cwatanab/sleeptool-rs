use crate::config::Config;
use crate::error::Result;
use crate::monitors::threshold::{evaluate, ThresholdState};
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{AudioProbe, PerformanceSnapshot, Platform};

/// 音声再生停止後のスリープ阻止猶予時間（秒）
const SOUND_DELAY_SECONDS: u64 = 15;
/// 音声検知のしきい値
const SOUND_THRESHOLD: f64 = 0.01;

pub struct SoundMonitor {
    state: ThresholdState,
}

impl SoundMonitor {
    pub fn new() -> Self {
        Self {
            state: ThresholdState::default(),
        }
    }
}

impl Monitor for SoundMonitor {
    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Sound
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.sound.enabled
    }

    fn sample(&mut self, _config: &Config, platform: &dyn Platform, _perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let rms = AudioProbe::current_sound_rms(platform)?;
        let inhibit = evaluate(&mut self.state, rms, SOUND_THRESHOLD, SOUND_DELAY_SECONDS);
        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: rms,
            threshold: SOUND_THRESHOLD,
        })
    }
}
