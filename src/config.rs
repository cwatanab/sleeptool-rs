use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{Result, SleepToolError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Simple,
    Detailed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThresholdConfig {
    pub enabled: bool,
    pub threshold: f64,
    pub delay_seconds: u64,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 1.0,
            delay_seconds: 600,
        }
    }
}

fn default_display_state_by_icon() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub mode: Mode,
    pub sleep_delay_seconds: u64,
    pub hibernate: bool,
    pub legacy_input: bool,
    pub sound_enabled: bool,
    pub auto_start: bool,
    pub display_off_on_sleep: bool,
    pub warn_before_sleep: bool,
    pub warn_sound_enabled: bool,
    #[serde(default = "default_display_state_by_icon")]
    pub display_state_by_icon: bool,
    pub resume_cooldown_seconds: u64,
    pub cpu: ThresholdConfig,
    pub network: ThresholdConfig,
    pub disk_write: ThresholdConfig,
    pub disk_read: ThresholdConfig,
    pub excluded_processes: Vec<String>,
    pub watched_processes: Vec<String>,
    pub watched_printers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Simple,
            sleep_delay_seconds: 600,
            hibernate: false,
            legacy_input: false,
            sound_enabled: true,
            auto_start: false,
            display_off_on_sleep: true,
            warn_before_sleep: true,
            warn_sound_enabled: true,
            display_state_by_icon: true,
            resume_cooldown_seconds: 60,
            cpu: ThresholdConfig {
                enabled: true,
                threshold: 1.0, // 1%
                delay_seconds: 600,
            },
            network: ThresholdConfig {
                enabled: true,
                threshold: 10000.0, // 10 KB/s
                delay_seconds: 600,
            },
            disk_write: ThresholdConfig {
                enabled: true,
                threshold: 10000.0, // 10 KB/s
                delay_seconds: 600,
            },
            disk_read: ThresholdConfig {
                enabled: false,
                threshold: 10000.0, // 10 KB/s
                delay_seconds: 600,
            },
            excluded_processes: vec![],
            watched_processes: vec![],
            watched_printers: vec![],
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| SleepToolError::Config(e.to_string()))?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| SleepToolError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn config_dir() -> std::path::PathBuf {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(local_app_data).join("SleepToolRust")
    }

    pub fn config_path() -> std::path::PathBuf {
        Self::config_dir().join("config.toml")
    }
}
