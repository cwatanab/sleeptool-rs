use crate::config::Config;
use crate::error::Result;
use crate::monitors::{
    cpu::CpuMonitor,
    disk::{DiskReadMonitor, DiskWriteMonitor},
    input::InputMonitor,
    network::NetworkMonitor,
    process::ProcessMonitor,
    sound::SoundMonitor,
    InhibitFactor, Monitor, MonitorState,
};
use crate::platform::{PerformanceProbe, Platform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineDecision {
    Sleep,
    Inhibit(MonitorState),
    Paused,
    /// スリープ復帰直後のクールダウン中。`remaining_secs` で残時間を返す。
    Cooldown { remaining_secs: u64 },
}

pub struct Engine {
    monitors: Vec<Box<dyn Monitor>>,
    paused: bool,
    last_resume: Option<std::time::Instant>,
}

impl Engine {
    pub fn new(_config: &Config) -> Self {
        Self {
            monitors: vec![
                Box::new(InputMonitor::new()),
                Box::new(CpuMonitor::new()),
                Box::new(NetworkMonitor::new()),
                Box::new(DiskWriteMonitor::new()),
                Box::new(SoundMonitor::new()),
                Box::new(DiskReadMonitor::new()),
                Box::new(ProcessMonitor::new()),
            ],
            paused: false,
            last_resume: None,
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn notify_resumed(&mut self) {
        self.last_resume = Some(std::time::Instant::now());
    }

    pub fn evaluate(&mut self, config: &Config, platform: &dyn Platform) -> Result<EngineDecision> {
        if self.paused {
            return Ok(EngineDecision::Paused);
        }

        if let Some(last_resume) = self.last_resume {
            let elapsed = last_resume.elapsed().as_secs();
            if elapsed < config.sleep.resume_cooldown_seconds {
                let remaining = config.sleep.resume_cooldown_seconds - elapsed;
                return Ok(EngineDecision::Cooldown { remaining_secs: remaining });
            }
        }

        let perf = PerformanceProbe::query_performance(platform)?;

        let mut inhibits: Vec<MonitorState> = Vec::new();
        for monitor in self.monitors.iter_mut() {
            if monitor.is_enabled(config) {
                let state = monitor.sample(config, platform, &perf)?;
                if state.inhibit {
                    inhibits.push(state);
                }
            }
        }

        if let Some(top) = inhibits
            .iter()
            .min_by_key(|s| s.factor.priority())
        {
            return Ok(EngineDecision::Inhibit(*top));
        }

        Ok(EngineDecision::Sleep)
    }
}
