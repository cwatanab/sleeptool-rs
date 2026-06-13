use crate::config::{Config, Mode};
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct PrinterMonitor;

impl PrinterMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Monitor for PrinterMonitor {
    fn name(&self) -> &'static str {
        "printer"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Printer
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.mode == Mode::Detailed && !config.watched_printers.is_empty()
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let has_jobs = platform.has_print_jobs(&config.watched_printers)?;
        Ok(MonitorState {
            inhibit: has_jobs,
            factor: self.default_factor(),
            value: if has_jobs { 1.0 } else { 0.0 },
            threshold: 1.0,
        })
    }
}
