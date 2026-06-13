use crate::config::Config;
use crate::error::Result;
use crate::platform::Platform;

pub mod cpu;
pub mod disk;
pub mod input;
pub mod network;
pub mod printer;
pub mod process;
pub mod sound;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InhibitFactor {
    Printer,
    Process,
    Sound,
    Cpu,
    Network,
    DiskRead,
    DiskWrite,
    Input,
}

impl InhibitFactor {
    pub fn priority(&self) -> u8 {
        match self {
            InhibitFactor::Printer => 1,
            InhibitFactor::Process => 2,
            InhibitFactor::Sound => 3,
            InhibitFactor::Cpu => 4,
            InhibitFactor::Network => 5,
            InhibitFactor::DiskRead => 6,
            InhibitFactor::DiskWrite => 7,
            InhibitFactor::Input => 8,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            InhibitFactor::Printer => "Printer",
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

#[derive(Debug, Clone)]
pub struct MonitorState {
    pub inhibit: bool,
    pub factor: InhibitFactor,
    pub value: f64,
    pub threshold: f64,
}

pub trait Monitor: Send {
    fn name(&self) -> &'static str;

    fn default_factor(&self) -> InhibitFactor;

    fn priority(&self) -> u8 {
        self.default_factor().priority()
    }

    fn is_enabled(&self, config: &Config) -> bool;
    fn sample(&mut self, config: &Config, platform: &dyn Platform) -> Result<MonitorState>;
}
