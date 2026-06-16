//! プラットフォームサブトレイトの宣言。
//!
//! 各サブトレイトは単一責務。集約 `Platform` トレイトは全サブトレイトの
//! 合成で、既存呼び出し側は `&dyn Platform` のまま使える。

use crate::error::Result;

use super::types::{PerformanceSnapshot, SleepType};

/// 性能メトリクス（CPU / ネットワーク / ディスク）をまとめて取得する。
pub trait PerformanceProbe {
    fn query_performance(&self) -> Result<PerformanceSnapshot>;
}

/// 最後の入力からの経過秒数を取得する。
///
/// `legacy_input == true` の場合、レガシーモード（キーボード／マウス
/// ポーリング）で取得する。`false` の場合は `GetLastInputInfo` を使う。
pub trait InputProbe {
    fn last_input_idle_seconds(&self, legacy_input: bool) -> Result<u64>;
}

/// 現在の出力音量を RMS で取得する。
pub trait AudioProbe {
    fn current_sound_rms(&self) -> Result<f64>;
}

/// 実行中のプロセス名一覧を取得する。
pub trait ProcessProbe {
    fn list_running_processes(&self) -> Result<Vec<String>>;
}

/// 電源制御（スリープ / ハイバネート / ディスプレイオフ）。
pub trait PowerControl {
    fn suspend(&self, sleep_type: SleepType, force: bool) -> Result<()>;
    fn turn_display_off(&self) -> Result<()>;
}

/// 通知（スリープ警告バルーン等）。
pub trait Notifier {
    /// スリープ警告を出し、ユーザーがキャンセルしたら `Ok(true)` を返す。
    fn show_sleep_warning(&self, seconds: u64, play_sound: bool) -> Result<bool>;
}

/// ログイン時の自動起動設定。
pub trait StartupControl {
    fn set_auto_start(&self, enable: bool) -> Result<()>;
    fn is_auto_start_enabled(&self) -> Result<bool>;
}

/// 全プラットフォーム能力の集約トレイト。
///
/// 既存コードの `&dyn Platform` を維持しつつ、内部はサブトレイトの合成。
/// 個別サブトレイトだけ実装したモックもサポート（テスト容易化）。
///
/// メソッドは持たない（デフォルト実装は用意しない）。呼び出し側は
/// UFCS（`PerformanceProbe::query_performance(platform)` 形式）で
/// 対応するサブトレイトを明示する。これにより曖昧性（multiple
/// applicable items in scope）を排除する。
pub trait Platform:
    PerformanceProbe
    + InputProbe
    + AudioProbe
    + ProcessProbe
    + PowerControl
    + Notifier
    + StartupControl
{
}
