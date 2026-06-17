use crate::config::Config;
use crate::error::Result;
use crate::platform::{PerformanceSnapshot, Platform};

pub mod cpu;
pub mod disk;
pub mod input;
pub mod network;
pub mod process;
pub mod sound;
pub mod threshold;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InhibitFactor {
    Process = 2,
    Sound = 3,
    Cpu = 4,
    Network = 5,
    DiskRead = 6,
    DiskWrite = 7,
    Input = 8,
}

impl InhibitFactor {
    pub fn priority(self) -> u8 { self as u8 }

    pub fn label(self) -> &'static str {
        match self {
            InhibitFactor::Process => "Process",
            InhibitFactor::Sound => "Sound",
            InhibitFactor::Cpu => "CPU",
            InhibitFactor::Network => "Network",
            InhibitFactor::DiskRead => "Disk Read",
            InhibitFactor::DiskWrite => "Disk Write",
            InhibitFactor::Input => "Input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonitorState {
    pub inhibit: bool,
    pub factor: InhibitFactor,
    pub value: f64,
    pub threshold: f64,
}

pub trait Monitor: Send {
    fn default_factor(&self) -> InhibitFactor;
    fn is_enabled(&self, config: &Config) -> bool;
    fn sample(&mut self, config: &Config, platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState>;
}
