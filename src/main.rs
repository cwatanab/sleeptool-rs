#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sleeptool_rs::cli::Cli;
use sleeptool_rs::config::{Config, LogLevel};
use sleeptool_rs::tracing;
use sleeptool_rs::engine::{Engine, EngineDecision};
use sleeptool_rs::logging;
use sleeptool_rs::monitors::InhibitFactor;
use sleeptool_rs::platform_win32::WindowsPlatform;
use sleeptool_rs::sleep;
use sleeptool_rs::state::{AppState, SharedState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn ensure_single_instance() -> anyhow::Result<()> {
    unsafe {
        let name: Vec<u16> = "SleepToolRs_SingleInstance\0".encode_utf16().collect();
        let _handle = windows::Win32::System::Threading::CreateMutexW(
            None,
            false,
            windows::core::PCWSTR(name.as_ptr()),
        )?;
        if windows::Win32::Foundation::GetLastError()
            == windows::Win32::Foundation::ERROR_ALREADY_EXISTS
        {
            let _ = windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                windows::Win32::Foundation::HWND::default(),
                windows::core::w!("SleepTool は既に起動しています。\nタスクトレイのアイコンから操作してください。"),
                windows::core::w!("SleepTool"),
                windows::Win32::UI::WindowsAndMessaging::MB_OK | windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION,
            );
            anyhow::bail!("Already running");
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    ensure_single_instance()?;
    let cli = Cli::parse();

    let config_path = Config::find_config_path();
    std::fs::create_dir_all(Config::config_dir())?;
    let mut config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        let default = Config::default();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        default.save(&config_path)?;
        default
    };

    if cli.hibernate { config.sleep.hibernate = true; }
    if cli.legacy_input { config.general.legacy_input = true; }

    if config.general.log_level != LogLevel::Off {
        logging::init_logging(&Config::config_dir().join("logs"))?;
    }
    logging::set_log_level(config.general.log_level);

    tracing::info!("SleepTool Rust starting...");

    let platform = Arc::new(WindowsPlatform::new()?);
    let state: SharedState = Arc::new(Mutex::new(AppState::new(config.clone(), config_path)));
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
    _config: Config,
) -> anyhow::Result<()> {
    let mut engine = Engine::new();
    let mut prev_factor: Option<InhibitFactor> = None;
    let mut was_paused = false;

    while running.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_secs(1));

        let (current_config, is_paused) = {
            let s = state.lock().unwrap();
            (s.config.clone(), s.paused)
        };

        engine.set_paused(is_paused);

        let decision = engine.evaluate(&current_config, platform.as_ref())?;
        let hwnd_val = {
            let mut s = state.lock().unwrap();
            s.current_decision = decision.clone();
            s.current_factor = match &decision {
                EngineDecision::Inhibit(ms) => Some(ms.factor),
                _ => None,
            };
            s.hwnd
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
                platform.reset_smoothing();
                prev_factor = None;
                was_paused = false;
            }
            EngineDecision::Inhibit(ms) => {
                if prev_factor != Some(ms.factor) {
                    if ms.factor == InhibitFactor::Input {
                        tracing::debug!("Sleep inhibited by Input");
                    } else {
                        tracing::debug!(
                            "Sleep inhibited by {} ({:.1} / {:.1})",
                            ms.factor.label(),
                            ms.value,
                            ms.threshold,
                        );
                    }
                }
                prev_factor = Some(ms.factor);
                was_paused = false;
            }
            EngineDecision::Paused => {
                if !was_paused { tracing::debug!("Monitoring paused"); }
                was_paused = true;
                prev_factor = None;
            }
            EngineDecision::Cooldown { remaining_secs } => {
                tracing::debug!("Resume cooldown active ({}s remaining)", remaining_secs);
                was_paused = false;
            }
        }
    }
    Ok(())
}
