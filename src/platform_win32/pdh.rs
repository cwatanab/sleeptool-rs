//! PDH (Performance Data Helper) 経由の性能カウンタ操作。

use windows::core::PCWSTR;
use windows::Win32::System::Performance::{
    PdhAddCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
    PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE,
};

use crate::error::Result;

use super::util::check_pdh;

/// PDH クエリハンドルを生成し、最初のデータ収集まで済ませて返す。
///
/// # Safety
///
/// 内部で Win32 PDH API を呼び出す。
pub unsafe fn open_query() -> Result<isize> {
    let mut query = 0isize;
    check_pdh(PdhOpenQueryW(None, 0, &mut query))?;
    Ok(query)
}

/// カウンタをクエリに追加し、ハンドルを返す。
///
/// # Safety
///
/// `query` は有効な PDH クエリハンドル。
pub unsafe fn add_counter(query: isize, path: &str) -> Result<isize> {
    let wpath: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let mut counter = 0isize;
    check_pdh(PdhAddCounterW(query, PCWSTR(wpath.as_ptr()), 0, &mut counter))?;
    Ok(counter)
}

/// カウンタ値を `f64` で取得する（既に collect 済み前提）。
///
/// # Safety
///
/// `counter` は有効な PDH カウンタハンドル。
pub unsafe fn get_value(counter: isize) -> Result<f64> {
    let mut value = PDH_FMT_COUNTERVALUE::default();
    check_pdh(PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value))?;
    Ok(value.Anonymous.doubleValue)
}

/// クエリのデータ収集を指示する。
///
/// # Safety
///
/// `query` は有効な PDH クエリハンドル。
pub unsafe fn collect(query: isize) -> Result<()> {
    check_pdh(PdhCollectQueryData(query))
}

/// 標準的な 4 つのカウンタ（CPU / Network / DiskRead / DiskWrite）を追加する。
/// 失敗したカウンタは `None` として返す（PDH 環境差を許容）。
///
/// # Safety
///
/// `query` は有効な PDH クエリハンドル。
pub unsafe fn add_standard_counters(query: isize) -> Result<StandardCounters> {
    Ok(StandardCounters {
        cpu: add_counter(query, r"\Processor(_Total)\% Processor Time").ok(),
        network: add_counter(query, r"\Network Interface(*)\Bytes Total/sec").ok(),
        disk_read: add_counter(query, r"\PhysicalDisk(_Total)\Disk Read Bytes/sec").ok(),
        disk_write: add_counter(query, r"\PhysicalDisk(_Total)\Disk Write Bytes/sec").ok(),
    })
}

pub struct StandardCounters {
    pub cpu: Option<isize>,
    pub network: Option<isize>,
    pub disk_read: Option<isize>,
    pub disk_write: Option<isize>,
}

/// カウンタ値を 0.0 フォールバック付きで取得する。
///
/// # Safety
///
/// `counter` は有効な PDH カウンタハンドル（`Some` の場合）。
pub unsafe fn try_get(counter: Option<isize>) -> f64 {
    counter.and_then(|c| get_value(c).ok()).unwrap_or(0.0)
}

/// カウンタから平滑化済みスナップショットを計算する。
///
/// 指数移動平均 (EMA) で滑らかにする。`alpha` が平滑化の強さ。
pub fn smooth(prev: f64, raw: f64, alpha: f64) -> f64 {
    alpha * raw + (1.0 - alpha) * prev
}


