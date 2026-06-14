use crate::config::Config;
use crate::engine::EngineDecision;
use crate::monitors::InhibitFactor;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub paused: bool,
    pub current_decision: EngineDecision,
    pub current_factor: Option<InhibitFactor>,
    pub hwnd: Option<isize>,
    pub settings_open: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            paused: false,
            current_decision: EngineDecision::Inhibit(InhibitFactor::Input),
            current_factor: Some(InhibitFactor::Input),
            hwnd: None,
            settings_open: false,
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
