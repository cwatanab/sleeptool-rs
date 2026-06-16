use sleeptool_rs::config::{Config, LogLevel};

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.sleep.delay_seconds, 600);
    assert!(config.sound.enabled);
}

#[test]
fn test_config_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let config = Config::default();
    config.save(&path).unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.sleep.delay_seconds, config.sleep.delay_seconds);
}

#[test]
fn process_watched_lowercased_on_load() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(
        &path,
        r#"
[process]
watched = ["Target.EXE", "OTHER.exe"]
excluded = ["Banned.EXE"]
"#,
    )
    .unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.process.watched, vec!["target.exe", "other.exe"]);
    assert_eq!(loaded.process.excluded, vec!["banned.exe"]);
}

#[test]
fn empty_process_lists_normalized_to_empty() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, "[process]\nwatched = []\nexcluded = []\n").unwrap();
    let loaded = Config::load(&path).unwrap();
    assert!(loaded.process.watched.is_empty());
    assert!(loaded.process.excluded.is_empty());
}

#[test]
fn partial_toml_missing_cpu_section_uses_threshold_struct_default() {
    // 現状の挙動: TOML に [cpu] セクションが**無い**場合、
    // 構造体 `ThresholdConfig::default()` (threshold=1.0) が使われる。
    // これは `Config::default()` (threshold=5.0) と齟齬しているため、
    // Phase 2 で CPU/Network 設定分離時にこの不整合を解消する。
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, "[sleep]\ndelay_seconds = 900\n").unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.sleep.delay_seconds, 900);
    assert!(loaded.sound.enabled);
    assert_eq!(loaded.cpu.threshold, 1.0);
}

#[test]
fn partial_toml_present_cpu_section_missing_field_uses_field_default() {
    // 現状の挙動: [cpu] セクションが**有り** threshold フィールドが無い場合、
    // フィールドに `#[serde(default = "...")]` を付けているので
    // `Config::default()` と同じ値 (threshold=5.0) が使われる。
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, "[cpu]\nenabled = false\n").unwrap();
    let loaded = Config::load(&path).unwrap();
    assert!(!loaded.cpu.enabled);
    assert_eq!(loaded.cpu.threshold, 5.0);
    assert_eq!(loaded.cpu.delay_seconds, 10);
}

#[test]
fn default_disk_config_has_read_disabled() {
    let cfg = Config::default();
    assert!(!cfg.disk.read_enabled);
    assert!(cfg.disk.write_enabled);
}

#[test]
fn general_defaults() {
    let cfg = Config::default();
    assert!(!cfg.general.legacy_input);
    assert!(!cfg.general.auto_start);
    assert!(cfg.general.display_off_on_sleep);
    assert!(cfg.general.display_state_by_icon);
    assert_eq!(cfg.general.log_level, LogLevel::Off);
}

#[test]
fn sleep_defaults() {
    let cfg = Config::default();
    assert!(!cfg.sleep.hibernate);
    assert!(cfg.sleep.warn_before_sleep);
    assert!(cfg.sleep.warn_sound_enabled);
    assert_eq!(cfg.sleep.resume_cooldown_seconds, 60);
}

#[test]
fn save_then_load_preserves_all_fields() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let mut original = Config::default();
    original.sleep.delay_seconds = 1234;
    original.sleep.hibernate = true;
    original.cpu.threshold = 12.5;
    original.process.watched = vec!["alpha.exe".to_string(), "beta.exe".to_string()];
    original.general.auto_start = true;
    original.save(&path).unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.sleep.delay_seconds, 1234);
    assert!(loaded.sleep.hibernate);
    assert_eq!(loaded.cpu.threshold, 12.5);
    assert_eq!(loaded.process.watched, vec!["alpha.exe", "beta.exe"]);
    assert!(loaded.general.auto_start);
}

#[test]
fn log_level_off_alias_works() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, r#"[general]
log_level = "none"
"#)
    .unwrap();
    let loaded = Config::load(&path).unwrap();
    assert_eq!(loaded.general.log_level, LogLevel::Off);
}

#[test]
fn config_path_is_under_localappdata() {
    let path = Config::config_path();
    assert!(path.ends_with("config.toml"));
    let dir = Config::config_dir();
    assert!(path.starts_with(dir));
}

#[test]
fn config_dir_returns_static_ref() {
    // 戻り値の型が `&'static Path` であることを強制する。
    // これにより内部で `OnceLock<PathBuf>` キャッシュしていることを示す。
    let p: &'static std::path::Path = Config::config_dir();
    assert!(p.ends_with("SleepToolRust"));
}

#[test]
fn config_dir_returns_same_pointer_across_calls() {
    // 同じ静的参照が返る → 内部でキャッシュされている。
    let p1: &'static std::path::Path = Config::config_dir();
    let p2: &'static std::path::Path = Config::config_dir();
    assert_eq!(p1 as *const _, p2 as *const _);
}
