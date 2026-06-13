use sleeptool_rs::config::{Config, Mode};

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.mode, Mode::Simple);
    assert_eq!(config.sleep_delay_seconds, 600);
    assert!(config.sound_enabled);
}

#[test]
fn test_config_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let config = Config::default();
    config.save(&path).unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.mode, config.mode);
    assert_eq!(loaded.sleep_delay_seconds, config.sleep_delay_seconds);
}
