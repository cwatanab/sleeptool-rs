use sleeptool_rs::config::Config;
use sleeptool_rs::monitors::{sound::SoundMonitor, InhibitFactor, Monitor, MonitorState};

struct DummyMonitor;

impl Monitor for DummyMonitor {
    fn name(&self) -> &'static str {
        "dummy"
    }

    fn default_factor(&self) -> InhibitFactor {
        InhibitFactor::Cpu
    }

    fn is_enabled(&self, _config: &Config) -> bool {
        true
    }

    fn sample(&mut self, _config: &Config, _platform: &dyn sleeptool_rs::platform::Platform) -> sleeptool_rs::error::Result<MonitorState> {
        Ok(MonitorState {
            inhibit: true,
            factor: InhibitFactor::Cpu,
            value: 10.0,
            threshold: 5.0,
        })
    }
}

#[test]
fn test_dummy_monitor() {
    let platform = sleeptool_rs::platform_win32::WindowsPlatform::new().unwrap();
    let mut m = DummyMonitor;
    let state = m.sample(&Config::default(), &platform).unwrap();
    assert!(state.inhibit);
    assert_eq!(state.factor, InhibitFactor::Cpu);
}

struct LoudPlatform;

impl sleeptool_rs::platform::Platform for LoudPlatform {
    fn last_input_idle_seconds(&self, _legacy_input: bool) -> sleeptool_rs::error::Result<u64> { Ok(0) }
    fn query_performance(&self) -> sleeptool_rs::error::Result<sleeptool_rs::platform::PerformanceSnapshot> {
        Ok(sleeptool_rs::platform::PerformanceSnapshot::default())
    }
    fn current_sound_rms(&self) -> sleeptool_rs::error::Result<f64> { Ok(0.1) }
    fn list_running_processes(&self) -> sleeptool_rs::error::Result<Vec<String>> { Ok(vec![]) }
    fn has_print_jobs(&self, _n: &[String]) -> sleeptool_rs::error::Result<bool> { Ok(false) }
    fn list_printers(&self) -> sleeptool_rs::error::Result<Vec<String>> { Ok(vec![]) }
    fn set_auto_start(&self, _e: bool) -> sleeptool_rs::error::Result<()> { Ok(()) }
    fn is_auto_start_enabled(&self) -> sleeptool_rs::error::Result<bool> { Ok(false) }
    fn suspend(&self, _t: sleeptool_rs::platform::SleepType, _f: bool) -> sleeptool_rs::error::Result<()> { Ok(()) }
    fn turn_display_off(&self) -> sleeptool_rs::error::Result<()> { Ok(()) }
    fn show_sleep_warning(&self, _s: u64, _p: bool) -> sleeptool_rs::error::Result<bool> { Ok(false) }
}

#[test]
fn test_sound_monitor_inhibits_when_loud() {
    let mut monitor = SoundMonitor::new();
    let config = Config::default();
    let state = monitor.sample(&config, &LoudPlatform).unwrap();
    assert!(state.inhibit);
    assert_eq!(state.factor, InhibitFactor::Sound);
}
