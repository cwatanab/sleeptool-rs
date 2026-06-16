use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{Result, SleepToolError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[serde(alias = "none")]
    Off,
    Error,
    Warn,
    Info,
    Debug,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThresholdConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_cpu_threshold")]
    pub threshold: f64,
    #[serde(default = "default_cpu_delay_seconds")]
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

fn default_cpu_threshold() -> f64 {
    5.0
}

fn default_cpu_delay_seconds() -> u64 {
    10
}

fn default_network_threshold() -> f64 {
    50_000.0
}

fn default_network_delay_seconds() -> u64 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SleepConfig {
    #[serde(default = "default_sleep_delay_seconds")]
    pub delay_seconds: u64,
    #[serde(default)]
    pub hibernate: bool,
    #[serde(default = "default_true")]
    pub warn_before_sleep: bool,
    #[serde(default = "default_true")]
    pub warn_sound_enabled: bool,
    #[serde(default = "default_resume_cooldown_seconds")]
    pub resume_cooldown_seconds: u64,
}

fn default_sleep_delay_seconds() -> u64 {
    600
}

fn default_resume_cooldown_seconds() -> u64 {
    60
}

impl Default for SleepConfig {
    fn default() -> Self {
        Self {
            delay_seconds: 600,
            hibernate: false,
            warn_before_sleep: true,
            warn_sound_enabled: true,
            resume_cooldown_seconds: 60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SoundConfig {
    pub enabled: bool,
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessConfig {
    pub watched: Vec<String>,
    pub excluded: Vec<String>,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            watched: vec![],
            excluded: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiskConfig {
    pub write_enabled: bool,
    pub write_threshold: f64,
    pub write_delay_seconds: u64,
    pub read_enabled: bool,
    pub read_threshold: f64,
    pub read_delay_seconds: u64,
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            write_enabled: true,
            write_threshold: 300000.0,
            write_delay_seconds: 10,
            read_enabled: false,
            read_threshold: 300000.0,
            read_delay_seconds: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    #[serde(default)]
    pub legacy_input: bool,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default = "default_true")]
    pub display_off_on_sleep: bool,
    #[serde(default = "default_true")]
    pub display_state_by_icon: bool,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
}

fn default_true() -> bool {
    true
}

fn default_log_level() -> LogLevel {
    LogLevel::Off
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            legacy_input: false,
            auto_start: false,
            display_off_on_sleep: true,
            display_state_by_icon: true,
            log_level: LogLevel::Off,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub sleep: SleepConfig,
    #[serde(default)]
    pub cpu: ThresholdConfig,
    #[serde(default)]
    pub network: ThresholdConfig,
    #[serde(default)]
    pub disk: DiskConfig,
    #[serde(default)]
    pub sound: SoundConfig,
    #[serde(default)]
    pub process: ProcessConfig,
    #[serde(default)]
    pub general: GeneralConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sleep: SleepConfig::default(),
            cpu: ThresholdConfig {
                enabled: true,
                threshold: 5.0,
                delay_seconds: 10,
            },
            network: ThresholdConfig {
                enabled: true,
                threshold: 50000.0,
                delay_seconds: 10,
            },
            disk: DiskConfig::default(),
            sound: SoundConfig::default(),
            process: ProcessConfig::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)
            .map_err(|e| SleepToolError::Config(e.to_string()))?;
        config.process.watched = config.process.watched.iter().map(|s| s.to_lowercase()).collect();
        config.process.excluded = config.process.excluded.iter().map(|s| s.to_lowercase()).collect();
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| SleepToolError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn config_dir() -> &'static std::path::Path {
        static DIR: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
        DIR.get_or_init(|| {
            let local_app_data = std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| ".".to_string());
            std::path::PathBuf::from(local_app_data).join("SleepToolRust")
        }).as_path()
    }

    pub fn config_path() -> std::path::PathBuf {
        Self::config_dir().join("config.toml")
    }
}
