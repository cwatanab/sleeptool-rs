use std::collections::HashSet;

use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{PerformanceSnapshot, Platform, ProcessProbe};

pub struct ProcessMonitor;

impl ProcessMonitor { pub fn new() -> Self { Self } }

impl Monitor for ProcessMonitor {
    fn default_factor(&self) -> InhibitFactor { InhibitFactor::Process }
    fn is_enabled(&self, config: &Config) -> bool { !config.process.watched.is_empty() }
    fn sample(&mut self, config: &Config, platform: &dyn Platform, _perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let running = ProcessProbe::list_running_processes(platform)?;
        let running_set: HashSet<&str> = running.iter().map(|s| s.as_str()).collect();
        let matched = config.process.watched.iter().any(|p| running_set.contains(p.as_str()));
        Ok(MonitorState { inhibit: matched, factor: InhibitFactor::Process, value: if matched { 1.0 } else { 0.0 }, threshold: 1.0 })
    }
}
