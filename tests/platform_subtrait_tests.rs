//! サブトレイト分離の確認: 個別のサブトレイトだけが実装された型を
//! 扱えることを示す。これによりプラットフォーム実装の関心の分離を保証する。

mod common;

use common::MockHandle;
use sleeptool_rs::error::Result;
use sleeptool_rs::platform::{
    AudioProbe, InputProbe, Notifier, PerformanceProbe, PerformanceSnapshot, Platform,
    PowerControl, ProcessProbe, SleepType, StartupControl,
};

struct OnlyPerf(f64);
impl PerformanceProbe for OnlyPerf {
    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        Ok(PerformanceSnapshot {
            cpu_percent: self.0,
            ..Default::default()
        })
    }
}

struct OnlyInput(u64);
impl InputProbe for OnlyInput {
    fn last_input_idle_seconds(&self, _legacy: bool) -> Result<u64> {
        Ok(self.0)
    }
}

struct OnlyAudio(f64);
impl AudioProbe for OnlyAudio {
    fn current_sound_rms(&self) -> Result<f64> {
        Ok(self.0)
    }
}

struct OnlyProcess(Vec<String>);
impl ProcessProbe for OnlyProcess {
    fn list_running_processes(&self) -> Result<Vec<String>> {
        Ok(self.0.clone())
    }
}

struct OnlyPower;
impl PowerControl for OnlyPower {
    fn suspend(&self, _t: SleepType, _force: bool) -> Result<()> {
        Ok(())
    }
    fn turn_display_off(&self) -> Result<()> {
        Ok(())
    }
}

struct OnlyNotifier;
impl Notifier for OnlyNotifier {
    fn show_sleep_warning(&self, _seconds: u64, _play_sound: bool) -> Result<bool> {
        Ok(false)
    }
}

struct OnlyStartup(bool);
impl StartupControl for OnlyStartup {
    fn set_auto_start(&self, _enable: bool) -> Result<()> {
        Ok(())
    }
    fn is_auto_start_enabled(&self) -> Result<bool> {
        Ok(self.0)
    }
}

#[test]
fn performance_probe_alone_is_usable() {
    let p = OnlyPerf(42.0);
    let s = p.query_performance().unwrap();
    assert_eq!(s.cpu_percent, 42.0);
}

#[test]
fn input_probe_alone_is_usable() {
    let p = OnlyInput(120);
    assert_eq!(p.last_input_idle_seconds(false).unwrap(), 120);
}

#[test]
fn audio_probe_alone_is_usable() {
    let p = OnlyAudio(0.5);
    assert_eq!(p.current_sound_rms().unwrap(), 0.5);
}

#[test]
fn process_probe_alone_is_usable() {
    let p = OnlyProcess(vec!["a.exe".into(), "b.exe".into()]);
    let procs = p.list_running_processes().unwrap();
    assert_eq!(procs, vec!["a.exe", "b.exe"]);
}

#[test]
fn power_control_alone_is_usable() {
    let p = OnlyPower;
    assert!(p.suspend(SleepType::Sleep, true).is_ok());
    assert!(p.turn_display_off().is_ok());
}

#[test]
fn notifier_alone_is_usable() {
    let p = OnlyNotifier;
    assert!(!p.show_sleep_warning(5, false).unwrap());
}

#[test]
fn startup_control_alone_is_usable() {
    let p = OnlyStartup(true);
    assert!(p.is_auto_start_enabled().unwrap());
    assert!(p.set_auto_start(false).is_ok());
}

/// `Platform` を実装する型は全サブトレイトを実装していなければならない。
/// MockHandle は全サブトレイトを実装しているのでこのチェックを通る。
#[test]
fn mock_handle_satisfies_full_platform() {
    fn assert_is_platform<T: Platform>() {}
    use crate::common::MockPlatform;
    assert_is_platform::<crate::common::MockHandle>();
    let _ = MockPlatform::new().idle(10).build();
}
