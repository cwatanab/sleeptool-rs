//! 共通の「しきい値 + delay hold-off」評価ロジック。
//!
//! CPU / Network / DiskWrite / DiskRead の各 Monitor は、値の取り出し元が違うだけで、
//! 「値がしきい値以上なら inhibit、直前のアクティブから `delay_seconds` 秒以内なら
//! ホールドオフとして inhibit を維持」という同じ判定を行う。
//! その共通ロジックと汎用モニター型をここに抽出する。

use std::time::Instant;

use crate::config::Config;
use crate::error::Result;
use crate::monitors::{InhibitFactor, Monitor, MonitorState};
use crate::platform::{PerformanceSnapshot, Platform};

#[derive(Default)]
pub struct ThresholdState {
    last_active: Option<Instant>,
}

impl ThresholdState {
    pub fn last_active(&self) -> Option<Instant> { self.last_active }
}

pub fn evaluate(
    state: &mut ThresholdState,
    value: f64,
    threshold: f64,
    delay_seconds: u64,
) -> bool {
    let now = Instant::now();
    let active = value >= threshold;
    if active { state.last_active = Some(now); }
    active || state.last_active
        .map(|t| now.duration_since(t).as_secs() < delay_seconds)
        .unwrap_or(false)
}

pub struct ThresholdMonitor {
    state: ThresholdState,
    factor: InhibitFactor,
    get_value: fn(&PerformanceSnapshot) -> f64,
    enabled: fn(&Config) -> bool,
    threshold: fn(&Config) -> f64,
    delay: fn(&Config) -> u64,
}

impl ThresholdMonitor {
    pub fn new(
        factor: InhibitFactor,
        get_value: fn(&PerformanceSnapshot) -> f64,
        enabled: fn(&Config) -> bool,
        threshold: fn(&Config) -> f64,
        delay: fn(&Config) -> u64,
    ) -> Self {
        Self { state: ThresholdState::default(), factor, get_value, enabled, threshold, delay }
    }

    pub fn factor(&self) -> InhibitFactor { self.factor }
}

impl Monitor for ThresholdMonitor {
    fn default_factor(&self) -> InhibitFactor { self.factor }
    fn is_enabled(&self, config: &Config) -> bool { (self.enabled)(config) }

    fn sample(&mut self, config: &Config, _platform: &dyn Platform, perf: &PerformanceSnapshot) -> Result<MonitorState> {
        let value = (self.get_value)(perf);
        let thr = (self.threshold)(config);
        let delay_secs = (self.delay)(config);
        let inhibit = evaluate(&mut self.state, value, thr, delay_secs);
        Ok(MonitorState { inhibit, factor: self.factor, value, threshold: thr })
    }
}
