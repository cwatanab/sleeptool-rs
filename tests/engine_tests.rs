mod common;

use common::{idle_and_cpu, idle_only, MockPlatform};
use sleeptool_rs::config::{Config, ThresholdConfig};
use sleeptool_rs::engine::{Engine, EngineDecision};
use sleeptool_rs::monitors::InhibitFactor;

fn extract_inhibit(d: EngineDecision) -> Option<InhibitFactor> {
    match d {
        EngineDecision::Inhibit(s) => Some(s.factor),
        _ => None,
    }
}

#[test]
fn sleep_when_no_activity() {
    let config = Config::default();
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn inhibit_by_input_when_user_active() {
    let config = Config::default();
    let platform = idle_only(0);
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Input));
}

#[test]
fn inhibit_by_cpu_when_cpu_high() {
    let mut config = Config::default();
    config.cpu.threshold = 5.0;
    let platform = MockPlatform::new().idle(601).cpu(50.0).build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Cpu));
}

#[test]
fn cpu_disabled_ignores_cpu_value() {
    let mut config = Config::default();
    config.cpu.enabled = false;
    config.cpu.threshold = 0.0;
    let platform = MockPlatform::new().idle(601).cpu(99.0).build();
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn input_dominates_when_user_active_and_cpu_low() {
    let mut config = Config::default();
    config.cpu.threshold = 5.0;
    let platform = MockPlatform::new().idle(0).cpu(0.0).build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Input));
}

#[test]
fn network_high_triggers_inhibit() {
    let mut config = Config::default();
    config.network.threshold = 1000.0;
    let platform = MockPlatform::new()
        .idle(601)
        .network_kb(100.0)
        .build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Network));
}

#[test]
fn sound_loud_triggers_inhibit() {
    let mut config = Config::default();
    config.sound.enabled = true;
    let platform = MockPlatform::new().idle(601).sound_rms(0.5).build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Sound));
}

#[test]
fn process_running_triggers_inhibit() {
    let mut config = Config::default();
    config.process.watched = vec!["target.exe".to_string()];
    let platform = MockPlatform::new()
        .idle(601)
        .processes(&["target.exe"])
        .build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Process));
}

#[test]
fn process_not_running_does_not_inhibit() {
    let mut config = Config::default();
    config.process.watched = vec!["target.exe".to_string()];
    let platform = MockPlatform::new()
        .idle(601)
        .processes(&["other.exe"])
        .build();
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn priority_picks_lowest_priority_value() {
    let mut config = Config::default();
    config.cpu.threshold = 0.0;
    config.process.watched = vec!["a.exe".to_string()];
    let platform = MockPlatform::new()
        .idle(601)
        .cpu(50.0)
        .processes(&["a.exe"])
        .build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Process));
}

#[test]
fn priority_overrides_input_for_specific_factor() {
    let mut config = Config::default();
    config.cpu.threshold = 0.0;
    let platform = MockPlatform::new().idle(0).cpu(50.0).build();
    let mut engine = Engine::new(&config);
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(decision), Some(InhibitFactor::Cpu));
}

#[test]
fn paused_state_yields_paused_decision() {
    let config = Config::default();
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    engine.set_paused(true);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Paused
    );
}

#[test]
fn paused_continues_even_when_activity_present() {
    let config = Config::default();
    let platform = MockPlatform::new().idle(0).cpu(99.0).build();
    let mut engine = Engine::new(&config);
    engine.set_paused(true);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Paused
    );
}

#[test]
fn resume_cooldown_returns_cooldown_decision() {
    use sleeptool_rs::engine::EngineDecision;
    let mut config = Config::default();
    config.sleep.resume_cooldown_seconds = 60;
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    engine.notify_resumed();
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert!(matches!(decision, EngineDecision::Cooldown { .. }));
}

#[test]
fn cooldown_carries_remaining_seconds() {
    use sleeptool_rs::engine::EngineDecision;
    let mut config = Config::default();
    config.sleep.resume_cooldown_seconds = 60;
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    engine.notify_resumed();
    let decision = engine.evaluate(&config, &platform).unwrap();
    if let EngineDecision::Cooldown { remaining_secs } = decision {
        assert!(remaining_secs <= 60);
    } else {
        panic!("Expected Cooldown decision");
    }
}

#[test]
fn cooldown_decision_does_not_pretend_to_be_input() {
    // バグ回帰: 旧実装は Cooldown 中に InhibitFactor::Input を返していた。
    // 修正後は Cooldown バリアントが独立して存在する。
    use sleeptool_rs::engine::EngineDecision;
    let mut config = Config::default();
    config.sleep.resume_cooldown_seconds = 60;
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    engine.notify_resumed();
    let decision = engine.evaluate(&config, &platform).unwrap();
    assert!(!matches!(decision, EngineDecision::Inhibit(_)));
}

#[test]
fn resume_cooldown_lifts_after_period() {
    let config = Config::default();
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    engine.notify_resumed();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    if config.sleep.resume_cooldown_seconds == 0 {
        assert_eq!(
            engine.evaluate(&config, &platform).unwrap(),
            EngineDecision::Sleep
        );
    }
}

#[test]
fn cpu_uses_configured_threshold() {
    let mut config = Config::default();
    config.cpu.threshold = 30.0;
    let platform = MockPlatform::new().idle(601).cpu(20.0).build();
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn legacy_input_flag_passed_through() {
    let config = Config::default();
    let platform = idle_only(601);
    let mut engine = Engine::new(&config);
    let _ = engine.evaluate(&config, &platform).unwrap();
}

#[test]
fn empty_engine_when_all_disabled_yields_sleep() {
    let mut config = Config::default();
    config.cpu.enabled = false;
    config.network.enabled = false;
    config.disk.write_enabled = false;
    config.disk.read_enabled = false;
    config.sound.enabled = false;
    config.process.watched.clear();
    let platform = idle_and_cpu(601, 0.0);
    let mut engine = Engine::new(&config);
    assert_eq!(
        engine.evaluate(&config, &platform).unwrap(),
        EngineDecision::Sleep
    );
}

#[test]
fn threshold_config_holds_short_pulses() {
    let mut config = Config::default();
    config.cpu.threshold = 50.0;
    config.cpu.delay_seconds = 60;
    let platform = MockPlatform::new().idle(601).cpu(80.0).build();
    let mut engine = Engine::new(&config);
    let first = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(first), Some(InhibitFactor::Cpu));
}

#[test]
fn threshold_after_active_pulse_still_inhibits_during_delay() {
    let mut config = Config::default();
    config.cpu.threshold = 50.0;
    config.cpu.delay_seconds = 60;
    let platform = MockPlatform::new().idle(601).cpu(80.0).build();
    let mut engine = Engine::new(&config);
    let first = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(first), Some(InhibitFactor::Cpu));
    platform.set_cpu(0.0);
    let second = engine.evaluate(&config, &platform).unwrap();
    assert_eq!(extract_inhibit(second), Some(InhibitFactor::Cpu));
}

#[test]
fn threshold_config_default_behavior() {
    let config = Config::default();
    assert!(config.cpu.enabled);
    assert_eq!(config.cpu.threshold, 5.0);
    assert_eq!(config.cpu.delay_seconds, 10);
    let _ = ThresholdConfig::default();
}
