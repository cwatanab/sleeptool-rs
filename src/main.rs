#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sleeptool_rs::cli::Cli;
use sleeptool_rs::config::Config;
use sleeptool_rs::tracing;
use sleeptool_rs::engine::{Engine, EngineDecision};
use sleeptool_rs::logging;
use sleeptool_rs::platform_win32::WindowsPlatform;
use sleeptool_rs::sleep;
use sleeptool_rs::state::{AppState, SharedState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config_path = Config::config_path();
    std::fs::create_dir_all(Config::config_dir())?;
    let mut config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        let default = Config::default();
        default.save(&config_path)?;
        default
    };

    if cli.hibernate {
        config.hibernate = true;
    }
    if cli.legacy_input {
        config.legacy_input = true;
    }

    logging::init_logging(&Config::config_dir().join("logs"))?;

    tracing::info!("SleepTool Rust starting...");

    let platform = Arc::new(WindowsPlatform::new()?);

    let state: SharedState = Arc::new(Mutex::new(AppState::new(config.clone())));
    let running = Arc::new(AtomicBool::new(true));

    let monitor_state = state.clone();
    let monitor_running = running.clone();
    let monitor_platform = platform.clone();
    let monitor_handle = std::thread::spawn(move || {
        if let Err(e) = run_monitor(monitor_state, monitor_running, monitor_platform, config) {
            tracing::error!("Monitor error: {}", e);
        }
    });

    let tray_state = state.clone();
    let tray_running = running.clone();
    let tray_platform = platform.clone();
    let tray_result = sleeptool_rs::tray::run_tray(tray_state, tray_running, tray_platform);

    if let Err(e) = tray_result {
        tracing::error!("Tray error: {:?}", e);
        running.store(false, Ordering::Relaxed);
        let _ = monitor_handle.join();
        return Err(e);
    }

    running.store(false, Ordering::Relaxed);
    let _ = monitor_handle.join();

    Ok(())
}

fn run_monitor(
    state: SharedState,
    running: Arc<AtomicBool>,
    platform: Arc<WindowsPlatform>,
    config: Config,
) -> anyhow::Result<()> {
    let mut engine = Engine::new(&config);

    while running.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_secs(1));

        let (current_config, is_paused) = {
            let state = state.lock().unwrap();
            (state.config.clone(), state.paused)
        };

        engine.set_paused(is_paused);

        let decision = engine.evaluate(&current_config, platform.as_ref())?;
        let hwnd_val = {
            let mut state = state.lock().unwrap();
            state.current_decision = decision.clone();
            state.current_factor = match &decision {
                EngineDecision::Inhibit(f) => Some(*f),
                _ => None,
            };
            state.hwnd
        };

        if let Some(hwnd) = hwnd_val {
            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    windows::Win32::Foundation::HWND(hwnd as *mut std::ffi::c_void),
                    sleeptool_rs::tray::WM_UPDATE_TRAY,
                    windows::Win32::Foundation::WPARAM(0),
                    windows::Win32::Foundation::LPARAM(0),
                );
            }
        }

        match decision {
            EngineDecision::Sleep => {
                tracing::info!("Conditions met, executing sleep");
                let _ = sleep::execute_sleep(platform.as_ref(), &current_config);
                engine.notify_resumed();
            }
            EngineDecision::Inhibit(f) => {
                tracing::debug!("Sleep inhibited by {:?}", f);
            }
            EngineDecision::Paused => {
                tracing::debug!("Monitoring paused");
            }
        }
    }

    Ok(())
}
