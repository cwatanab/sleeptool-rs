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
    Cooldown { remaining_secs: u64 },
}

macro_rules! any_monitor {
    ($($variant:ident($inner:ty)),+ $(,)?) => {
        enum AnyMonitor {
            $($variant($inner)),+
        }

        impl Monitor for AnyMonitor {
            fn default_factor(&self) -> InhibitFactor {
                match self { $( Self::$variant(m) => m.default_factor(), )+ }
            }
            fn is_enabled(&self, config: &Config) -> bool {
                match self { $( Self::$variant(m) => m.is_enabled(config), )+ }
            }
            fn sample(&mut self, config: &Config, platform: &dyn Platform, perf: &crate::platform::PerformanceSnapshot) -> Result<MonitorState> {
                match self { $( Self::$variant(m) => m.sample(config, platform, perf), )+ }
            }
        }
    };
}

any_monitor! {
    Input(InputMonitor),
    Cpu(CpuMonitor),
    Network(NetworkMonitor),
    DiskWrite(DiskWriteMonitor),
    Sound(SoundMonitor),
    DiskRead(DiskReadMonitor),
    Process(ProcessMonitor),
}

pub struct Engine {
    monitors: [AnyMonitor; 7],
    paused: bool,
    last_resume: Option<std::time::Instant>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            monitors: [
                AnyMonitor::Input(InputMonitor::new()),
                AnyMonitor::Cpu(CpuMonitor::new()),
                AnyMonitor::Network(NetworkMonitor::new()),
                AnyMonitor::DiskWrite(DiskWriteMonitor::new()),
                AnyMonitor::Sound(SoundMonitor::new()),
                AnyMonitor::DiskRead(DiskReadMonitor::new()),
                AnyMonitor::Process(ProcessMonitor::new()),
            ],
            paused: false,
            last_resume: None,
        }
    }

    pub fn set_paused(&mut self, paused: bool) { self.paused = paused; }
    pub fn is_paused(&self) -> bool { self.paused }
    pub fn notify_resumed(&mut self) { self.last_resume = Some(std::time::Instant::now()); }

    pub fn evaluate(&mut self, config: &Config, platform: &dyn Platform) -> Result<EngineDecision> {
        if self.paused { return Ok(EngineDecision::Paused); }
        if let Some(last_resume) = self.last_resume {
            let elapsed = last_resume.elapsed().as_secs();
            if elapsed < config.sleep.resume_cooldown_seconds {
                return Ok(EngineDecision::Cooldown { remaining_secs: config.sleep.resume_cooldown_seconds - elapsed });
            }
        }
        let perf = PerformanceProbe::query_performance(platform)?;
        let mut top: Option<MonitorState> = None;
        for monitor in &mut self.monitors {
            if monitor.is_enabled(config) {
                let state = monitor.sample(config, platform, &perf)?;
                if state.inhibit && top.map_or(true, |t| state.factor.priority() < t.factor.priority()) {
                    top = Some(state);
                }
            }
        }
        if let Some(ms) = top { return Ok(EngineDecision::Inhibit(ms)); }
        Ok(EngineDecision::Sleep)
    }
}
