use sleeptool_rs::config::{Config, ThresholdConfig};
use sleeptool_rs::engine::{Engine, EngineDecision};
use sleeptool_rs::error::Result;
use sleeptool_rs::platform::{Platform, PerformanceSnapshot, SleepType};

struct MockPlatform {
    idle_seconds: u64,
}

impl Platform for MockPlatform {
    fn last_input_idle_seconds(&self, _legacy_input: bool) -> Result<u64> {
        Ok(self.idle_seconds)
    }

    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        Ok(PerformanceSnapshot::default())
    }

    fn current_sound_rms(&self) -> Result<f64> {
        Ok(0.0)
    }

    fn list_running_processes(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn has_print_jobs(&self, _printer_names: &[String]) -> Result<bool> {
        Ok(false)
    }

    fn list_printers(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn set_auto_start(&self, _enable: bool) -> Result<()> {
        Ok(())
    }

    fn is_auto_start_enabled(&self) -> Result<bool> {
        Ok(false)
    }

    fn suspend(&self, _sleep_type: SleepType, _force: bool) -> Result<()> {
        Ok(())
    }

    fn turn_display_off(&self) -> Result<()> {
        Ok(())
    }

    fn show_sleep_warning(&self, _s: u64, _p: bool) -> Result<bool> {
        Ok(false)
    }
}

#[test]
fn test_engine_sleep_when_idle() {
    let config = Config::default();
    let platform = MockPlatform { idle_seconds: 601 };
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(decision, EngineDecision::Sleep);
}

#[test]
fn test_engine_inhibit_when_active() {
    let mut config = Config::default();
    config.cpu = ThresholdConfig {
        enabled: true,
        threshold: 0.0,
        delay_seconds: 600,
    };
    let platform = MockPlatform { idle_seconds: 601 };
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert!(matches!(decision, EngineDecision::Inhibit(_)));
}
