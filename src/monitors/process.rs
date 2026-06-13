use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Monitor for ProcessMonitor {
    fn name(&self) -> &'static str {
        "process"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Process
    }

    fn is_enabled(&self, config: &Config) -> bool {
        !config.watched_processes.is_empty()
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let running = platform.list_running_processes()?;
        let matched = config
            .watched_processes
            .iter()
            .any(|p| {
                let p_lower = p.to_lowercase();
                running.contains(&p_lower)
            });
        Ok(MonitorState {
            inhibit: matched,
            factor: self.default_factor(),
            value: if matched { 1.0 } else { 0.0 },
            threshold: 1.0,
        })
    }
}
