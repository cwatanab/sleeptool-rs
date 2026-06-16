//! 統合テスト共通の `Platform` モック。
//!
//! ビルダーパターンで `Platform` の各メソッドを差し替え可能にし、
//! テスト本体は「どの値を返すか」だけを書けばよい。
//!
//! 実行中の値変更は `MockHandle::set_*` で行う。複数サンプル間の
//! 状態遷移（例: アクティブ→非アクティブ）を再現するために使う。
//!
//! ```ignore
//! use crate::common::MockPlatform;
//! let platform = MockPlatform::new().idle(601).cpu(0.0).build();
//! platform.set_cpu(80.0); // 次の sample() から 80.0 が返る
//! ```

use std::sync::{Arc, Mutex};

use sleeptool_rs::error::Result;
use sleeptool_rs::platform::{
    AudioProbe, InputProbe, Notifier, PerformanceProbe, PerformanceSnapshot, Platform,
    PowerControl, ProcessProbe, SleepType, StartupControl,
};

#[derive(Debug, Clone)]
pub struct MockPlatform {
    idle_seconds: u64,
    cpu_percent: f64,
    network_bytes_per_sec: f64,
    disk_write_bytes_per_sec: f64,
    disk_read_bytes_per_sec: f64,
    sound_rms: f64,
    processes: Vec<String>,
}

impl Default for MockPlatform {
    fn default() -> Self {
        Self {
            idle_seconds: 0,
            cpu_percent: 0.0,
            network_bytes_per_sec: 0.0,
            disk_write_bytes_per_sec: 0.0,
            disk_read_bytes_per_sec: 0.0,
            sound_rms: 0.0,
            processes: Vec::new(),
        }
    }
}

impl MockPlatform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn idle(mut self, sec: u64) -> Self {
        self.idle_seconds = sec;
        self
    }

    pub fn cpu(mut self, percent: f64) -> Self {
        self.cpu_percent = percent;
        self
    }

    pub fn network_kb(mut self, kb_per_sec: f64) -> Self {
        self.network_bytes_per_sec = kb_per_sec * 1024.0;
        self
    }

    pub fn disk_write_kb(mut self, kb_per_sec: f64) -> Self {
        self.disk_write_bytes_per_sec = kb_per_sec * 1024.0;
        self
    }

    pub fn disk_read_kb(mut self, kb_per_sec: f64) -> Self {
        self.disk_read_bytes_per_sec = kb_per_sec * 1024.0;
        self
    }

    pub fn sound_rms(mut self, rms: f64) -> Self {
        self.sound_rms = rms;
        self
    }

    pub fn processes(mut self, names: &[&str]) -> Self {
        self.processes = names.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> MockHandle {
        MockHandle {
            inner: Arc::new(Mutex::new(self)),
        }
    }
}

/// `Platform` トレイトを実装するハンドル。`&dyn Platform` として渡せる。
/// 内部状態が `Arc<Mutex<...>>` なので、テスト中に `set_*` で値を変更できる。
#[derive(Clone)]
pub struct MockHandle {
    inner: Arc<Mutex<MockPlatform>>,
}

impl MockHandle {
    pub fn set_idle(&self, sec: u64) {
        self.inner.lock().unwrap().idle_seconds = sec;
    }

    pub fn set_cpu(&self, percent: f64) {
        self.inner.lock().unwrap().cpu_percent = percent;
    }

    pub fn set_network_kb(&self, kb_per_sec: f64) {
        self.inner.lock().unwrap().network_bytes_per_sec = kb_per_sec * 1024.0;
    }

    pub fn set_disk_write_kb(&self, kb_per_sec: f64) {
        self.inner.lock().unwrap().disk_write_bytes_per_sec = kb_per_sec * 1024.0;
    }

    pub fn set_disk_read_kb(&self, kb_per_sec: f64) {
        self.inner.lock().unwrap().disk_read_bytes_per_sec = kb_per_sec * 1024.0;
    }

    pub fn set_sound_rms(&self, rms: f64) {
        self.inner.lock().unwrap().sound_rms = rms;
    }

    pub fn set_processes(&self, names: &[&str]) {
        self.inner.lock().unwrap().processes = names.iter().map(|s| s.to_string()).collect();
    }
}

impl Platform for MockHandle {}

impl PerformanceProbe for MockHandle {
    fn query_performance(&self) -> Result<PerformanceSnapshot> {
        let g = self.inner.lock().unwrap();
        Ok(PerformanceSnapshot {
            cpu_percent: g.cpu_percent,
            network_bytes_per_sec: g.network_bytes_per_sec,
            disk_write_bytes_per_sec: g.disk_write_bytes_per_sec,
            disk_read_bytes_per_sec: g.disk_read_bytes_per_sec,
        })
    }
}

impl InputProbe for MockHandle {
    fn last_input_idle_seconds(&self, _legacy_input: bool) -> Result<u64> {
        Ok(self.inner.lock().unwrap().idle_seconds)
    }
}

impl AudioProbe for MockHandle {
    fn current_sound_rms(&self) -> Result<f64> {
        Ok(self.inner.lock().unwrap().sound_rms)
    }
}

impl ProcessProbe for MockHandle {
    fn list_running_processes(&self) -> Result<Vec<String>> {
        Ok(self.inner.lock().unwrap().processes.clone())
    }
}

impl PowerControl for MockHandle {
    fn suspend(&self, _sleep_type: SleepType, _force: bool) -> Result<()> {
        Ok(())
    }

    fn turn_display_off(&self) -> Result<()> {
        Ok(())
    }
}

impl Notifier for MockHandle {
    fn show_sleep_warning(&self, _seconds: u64, _play_sound: bool) -> Result<bool> {
        Ok(false)
    }
}

impl StartupControl for MockHandle {
    fn set_auto_start(&self, _enable: bool) -> Result<()> {
        Ok(())
    }

    fn is_auto_start_enabled(&self) -> Result<bool> {
        Ok(false)
    }
}

/// 旧テストとの互換用ショートカット。
pub fn idle_only(sec: u64) -> MockHandle {
    MockPlatform::new().idle(sec).build()
}

/// 旧 `FakePlatform { idle, cpu }` 相当を構築。
pub fn idle_and_cpu(idle: u64, cpu: f64) -> MockHandle {
    MockPlatform::new().idle(idle).cpu(cpu).build()
}

/// 旧 `LoudPlatform` 相当（音量大）。
pub fn loud() -> MockHandle {
    MockPlatform::new().idle(0).sound_rms(0.1).build()
}
