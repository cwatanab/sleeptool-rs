use crate::config::Config;
use crate::engine::EngineDecision;
use crate::monitors::{InhibitFactor, MonitorState};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub config_path: PathBuf,
    pub paused: bool,
    pub current_decision: EngineDecision,
    pub current_factor: Option<InhibitFactor>,
    pub hwnd: Option<isize>,
}

impl AppState {
    pub fn new(config: Config, config_path: PathBuf) -> Self {
        Self {
            config: Arc::new(config),
            config_path,
            paused: false,
            current_decision: EngineDecision::Inhibit(MonitorState {
                inhibit: true,
                factor: InhibitFactor::Input,
                value: 0.0,
                threshold: 0.0,
            }),
            current_factor: Some(InhibitFactor::Input),
            hwnd: None,
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
