use crate::config::Config;
use crate::error::Result;
use crate::monitors::{
    cpu::CpuMonitor, disk::DiskReadMonitor, disk::DiskWriteMonitor, input::InputMonitor,
    network::NetworkMonitor, process::ProcessMonitor,
    sound::SoundMonitor, InhibitFactor, Monitor, MonitorState,
};
use crate::platform::Platform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineDecision {
    Sleep,
    Inhibit(InhibitFactor),
    Paused,
}

pub struct Engine {
    monitors: Vec<Box<dyn Monitor>>,
    paused: bool,
    last_resume: Option<std::time::Instant>,
}

impl Engine {
    pub fn new(_config: &Config) -> Self {
        let monitors: Vec<Box<dyn Monitor>> = vec![
            Box::new(InputMonitor::new()),
            Box::new(CpuMonitor::new()),
            Box::new(NetworkMonitor::new()),
            Box::new(DiskWriteMonitor::new()),
            Box::new(SoundMonitor::new()),
            Box::new(DiskReadMonitor::new()),
            Box::new(ProcessMonitor::new()),
        ];

        Self {
            monitors,
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
            if last_resume.elapsed().as_secs() < config.resume_cooldown_seconds {
                return Ok(EngineDecision::Inhibit(InhibitFactor::Input));
            }
        }

        let mut inhibits: Vec<MonitorState> = Vec::new();
        for monitor in &mut self.monitors {
            if !monitor.is_enabled(config) {
                continue;
            }
            let state = monitor.sample(config, platform)?;
            if state.inhibit {
                inhibits.push(state);
            }
        }

        if let Some(top) = inhibits
            .iter()
            .min_by_key(|s| s.factor.priority())
        {
            return Ok(EngineDecision::Inhibit(top.factor));
        }

        Ok(EngineDecision::Sleep)
    }
}
