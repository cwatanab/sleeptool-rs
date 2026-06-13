use crate::config::{Config, Mode};
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::Platform;

pub struct DiskWriteMonitor {
    last_active: Option<std::time::Instant>,
}
pub struct DiskReadMonitor {
    last_active: Option<std::time::Instant>,
}

impl DiskWriteMonitor {
    pub fn new() -> Self {
        Self { last_active: None }
    }
}

impl Monitor for DiskWriteMonitor {
    fn name(&self) -> &'static str {
        "disk_write"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::DiskWrite
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.disk_write.enabled
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let perf = platform.query_performance()?;
        let cfg = &config.disk_write;
        let active = perf.disk_write_bytes_per_sec >= cfg.threshold;

        let now = std::time::Instant::now();
        if active {
            self.last_active = Some(now);
        }

        let inhibit = if active {
            true
        } else if let Some(last) = self.last_active {
            now.duration_since(last).as_secs() < cfg.delay_seconds
        } else {
            false
        };

        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.disk_write_bytes_per_sec,
            threshold: cfg.threshold,
        })
    }
}

impl DiskReadMonitor {
    pub fn new() -> Self {
        Self { last_active: None }
    }
}

impl Monitor for DiskReadMonitor {
    fn name(&self) -> &'static str {
        "disk_read"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::DiskRead
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.mode == Mode::Detailed && config.disk_read.enabled
    }

    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState> {
        let perf = platform.query_performance()?;
        let cfg = &config.disk_read;
        let active = perf.disk_read_bytes_per_sec >= cfg.threshold;

        let now = std::time::Instant::now();
        if active {
            self.last_active = Some(now);
        }

        let inhibit = if active {
            true
        } else if let Some(last) = self.last_active {
            now.duration_since(last).as_secs() < cfg.delay_seconds
        } else {
            false
        };

        Ok(MonitorState {
            inhibit,
            factor: self.default_factor(),
            value: perf.disk_read_bytes_per_sec,
            threshold: cfg.threshold,
        })
    }
}
