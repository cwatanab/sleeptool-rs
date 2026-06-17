mod common;

use common::idle_and_cpu;
use sleeptool_rs::config::Config;
use sleeptool_rs::engine::{Engine, EngineDecision};

#[test]
fn test_sleep_when_completely_idle() {
    let config = Config::default();
    let platform = idle_and_cpu(601, 0.0);
    let mut engine = Engine::new();
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn test_inhibit_when_cpu_high() {
    let mut config = Config::default();
    config.cpu.threshold = 1.0;
    let platform = idle_and_cpu(601, 5.0);
    let mut engine = Engine::new();
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert!(matches!(decision, EngineDecision::Inhibit(_)));
}
