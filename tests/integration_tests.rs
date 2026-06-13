use sleeptool_rs::config::Config;
use sleeptool_rs::engine::{Engine, EngineDecision};
use sleeptool_rs::error::Result;
use sleeptool_rs::platform::{Platform, PerformanceSnapshot, SleepType};

struct FakePlatform {
    idle: u64,
    cpu: f64,
}

impl Platform for FakePlatform {
    fn last_input_idle_seconds(&self, _legacy_input: bool) -> Result<u64> {
        Ok(self.idle)
    }

    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        Ok(PerformanceSnapshot {
            cpu_percent: self.cpu,
            ..Default::default()
        })
    }

    fn current_sound_rms(&self) -> Result<f64> {
        Ok(0.0)
    }

    fn list_running_processes(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn has_print_jobs(&self, _names: &[String]) -> Result<bool> {
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

    fn suspend(&self, _t: SleepType, _f: bool) -> Result<()> {
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
fn test_sleep_when_completely_idle() {
    let config = Config::default();
    let platform = FakePlatform { idle: 601, cpu: 0.0 };
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn test_inhibit_when_cpu_high() {
    let mut config = Config::default();
    config.cpu.threshold = 1.0;
    let platform = FakePlatform { idle: 601, cpu: 5.0 };
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert!(matches!(decision, EngineDecision::Inhibit(_)));
}
