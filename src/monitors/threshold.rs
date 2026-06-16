//! 共通の「しきい値 + delay hold-off」評価ロジック。
//!
//! CPU / Network / DiskWrite / DiskRead の各 Monitor は、値の取り出し元が違うだけで、
//! 「値がしきい値以上なら inhibit、直前のアクティブから `delay_seconds` 秒以内なら
//! ホールドオフとして inhibit を維持」という同じ判定を行う。
//! その共通ロジックをここに抽出する。

use std::time::Instant;

/// しきい値系 Monitor のホールドオフ状態。
#[derive(Default)]
pub struct ThresholdState {
    last_active: Option<Instant>,
}

impl ThresholdState {
    pub fn last_active(&self) -> Option<Instant> {
        self.last_active
    }
}

/// 「値が `threshold` 以上か、直前アクティブから `delay_seconds` 秒以内か」を判定。
///
/// - `value >= threshold` なら `last_active` を `now` に更新して `true` を返す。
/// - `value < threshold` でも `now - last_active < delay_seconds` なら `true`。
/// - それ以外は `false`。
pub fn evaluate(
    state: &mut ThresholdState,
    value: f64,
    threshold: f64,
    delay_seconds: u64,
) -> bool {
    let now = Instant::now();
    let active = value >= threshold;
    if active {
        state.last_active = Some(now);
    }
    active
        || state
            .last_active
            .map(|t| now.duration_since(t).as_secs() < delay_seconds)
            .unwrap_or(false)
}
