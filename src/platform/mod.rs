//! プラットフォーム抽象化レイヤー。
//!
//! サブトレイトに分割することで、テストモックが関心事ごとに最小実装できる。
//! 集約 `Platform` トレイトは全サブトレイトの合成で、既存呼び出し側は
//! `&dyn Platform` のまま使える。

mod traits;
mod types;

pub use traits::{
    AudioProbe, InputProbe, Notifier, PerformanceProbe, Platform, PowerControl, ProcessProbe,
    StartupControl,
};
pub use types::{InputIdleInfo, PerformanceSnapshot, SleepType};
