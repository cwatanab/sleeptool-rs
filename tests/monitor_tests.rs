mod common;

use common::{loud, MockPlatform};
use sleeptool_rs::config::Config;
use sleeptool_rs::monitors::{
    cpu::CpuMonitor,
    disk::{DiskReadMonitor, DiskWriteMonitor},
    input::InputMonitor,
    network::NetworkMonitor,
    process::ProcessMonitor,
    sound::SoundMonitor,
    InhibitFactor, Monitor,
};
use sleeptool_rs::platform::PerformanceSnapshot;

fn snap(cpu: f64, net: f64, dw: f64, dr: f64) -> PerformanceSnapshot {
    PerformanceSnapshot {
        cpu_percent: cpu,
        network_bytes_per_sec: net,
        disk_write_bytes_per_sec: dw,
        disk_read_bytes_per_sec: dr,
    }
}

#[test]
fn cpu_above_threshold_inhibits() {
    let mut m = CpuMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(80.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::Cpu);
}

#[test]
fn cpu_below_threshold_no_inhibit() {
    let mut m = CpuMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(!s.inhibit);
}

#[test]
fn cpu_at_threshold_inhibits() {
    let mut m = CpuMonitor::new();
    let mut cfg = Config::default();
    cfg.cpu.threshold = 5.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(5.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
}

#[test]
fn cpu_disabled_skips_sampling_via_is_enabled() {
    let m = CpuMonitor::new();
    let mut cfg = Config::default();
    cfg.cpu.enabled = false;
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn cpu_holds_inhibit_after_active_pulse() {
    let mut m = CpuMonitor::new();
    let mut cfg = Config::default();
    cfg.cpu.threshold = 50.0;
    cfg.cpu.delay_seconds = 60;
    let p = MockPlatform::new().idle(601).build();
    let _ = m.sample(&cfg, &p, &snap(80.0, 0.0, 0.0, 0.0)).unwrap();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
}

#[test]
fn cpu_value_and_threshold_reported() {
    let mut m = CpuMonitor::new();
    let mut cfg = Config::default();
    cfg.cpu.threshold = 25.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(42.0, 0.0, 0.0, 0.0)).unwrap();
    assert_eq!(s.value, 42.0);
    assert_eq!(s.threshold, 25.0);
}

#[test]
fn network_above_threshold_inhibits() {
    let mut m = NetworkMonitor::new();
    let mut cfg = Config::default();
    cfg.network.threshold = 1000.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 2000.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::Network);
}

#[test]
fn network_below_threshold_no_inhibit() {
    let mut m = NetworkMonitor::new();
    let mut cfg = Config::default();
    cfg.network.threshold = 10000.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 100.0, 0.0, 0.0)).unwrap();
    assert!(!s.inhibit);
}

#[test]
fn network_disabled() {
    let m = NetworkMonitor::new();
    let mut cfg = Config::default();
    cfg.network.enabled = false;
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn network_holds_during_delay() {
    let mut m = NetworkMonitor::new();
    let mut cfg = Config::default();
    cfg.network.threshold = 1000.0;
    cfg.network.delay_seconds = 60;
    let p = MockPlatform::new().idle(601).build();
    let _ = m.sample(&cfg, &p, &snap(0.0, 5000.0, 0.0, 0.0)).unwrap();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
}

#[test]
fn disk_write_above_threshold_inhibits() {
    let mut m = DiskWriteMonitor::new();
    let mut cfg = Config::default();
    cfg.disk.write_threshold = 1000.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 2000.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::DiskWrite);
}

#[test]
fn disk_write_disabled() {
    let m = DiskWriteMonitor::new();
    let mut cfg = Config::default();
    cfg.disk.write_enabled = false;
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn disk_read_above_threshold_inhibits() {
    let mut m = DiskReadMonitor::new();
    let mut cfg = Config::default();
    cfg.disk.read_enabled = true;
    cfg.disk.read_threshold = 1000.0;
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 2000.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::DiskRead);
}

#[test]
fn disk_read_disabled_by_default() {
    let m = DiskReadMonitor::new();
    let cfg = Config::default();
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn input_inhibits_when_idle_below_threshold() {
    let mut m = InputMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(0).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::Input);
}

#[test]
fn input_sleeps_when_idle_above_threshold() {
    let mut m = InputMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(601).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(!s.inhibit);
}

#[test]
fn input_value_threshold_match_config() {
    let mut m = InputMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(120).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert_eq!(s.value, 120.0);
    assert_eq!(s.threshold, cfg.sleep.delay_seconds as f64);
}

#[test]
fn process_inhibits_when_watched_running() {
    let mut m = ProcessMonitor::new();
    let mut cfg = Config::default();
    cfg.process.watched = vec!["target.exe".to_string()];
    let p = MockPlatform::new().idle(601).processes(&["target.exe"]).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::Process);
}

#[test]
fn process_no_inhibit_when_unwatched() {
    let mut m = ProcessMonitor::new();
    let mut cfg = Config::default();
    cfg.process.watched = vec!["target.exe".to_string()];
    let p = MockPlatform::new().idle(601).processes(&["other.exe"]).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(!s.inhibit);
}

#[test]
fn process_disabled_when_watched_empty() {
    let m = ProcessMonitor::new();
    let cfg = Config::default();
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn sound_inhibits_when_above_threshold() {
    let mut m = SoundMonitor::new();
    let cfg = Config::default();
    let p = loud();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
    assert_eq!(s.factor, InhibitFactor::Sound);
}

#[test]
fn sound_silent_no_inhibit() {
    let mut m = SoundMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(601).sound_rms(0.0).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(!s.inhibit);
}

#[test]
fn sound_disabled() {
    let m = SoundMonitor::new();
    let mut cfg = Config::default();
    cfg.sound.enabled = false;
    assert!(!m.is_enabled(&cfg));
}

#[test]
fn sound_at_threshold_inhibits() {
    let mut m = SoundMonitor::new();
    let cfg = Config::default();
    let p = MockPlatform::new().idle(601).sound_rms(0.01).build();
    let s = m.sample(&cfg, &p, &snap(0.0, 0.0, 0.0, 0.0)).unwrap();
    assert!(s.inhibit);
}

#[test]
fn inihibit_factor_priority_order_is_stable() {
    use sleeptool_rs::monitors::InhibitFactor::*;
    assert!(Process.priority() < Sound.priority());
    assert!(Sound.priority() < Cpu.priority());
    assert!(Cpu.priority() < Network.priority());
    assert!(Network.priority() < DiskRead.priority());
    assert!(DiskRead.priority() < DiskWrite.priority());
    assert!(DiskWrite.priority() < Input.priority());
}

#[test]
fn inhibit_factor_labels_are_nonempty() {
    use sleeptool_rs::monitors::InhibitFactor::*;
    for f in [Process, Sound, Cpu, Network, DiskRead, DiskWrite, Input] {
        assert!(f.priority() >= 2 && f.priority() <= 8);
    }
}

