//! カンマ区切り文字列のヘルパ。
//!
//! プロセス監視リスト（`watched` / `excluded`）の UI 編集と
//! 内部データ (`Vec<String>`) の橋渡しに使う。

/// カンマ区切り文字列を `Vec<String>` に分割する。
/// - 前後の空白をトリム
/// - 空要素を除去
/// - 小文字化
pub fn parse_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_lowercase())
        .filter(|item| !item.is_empty())
        .collect()
}

/// カンマ区切り文字列にアイテムを追加する。
/// - 既存アイテムと大文字小文字を無視して重複チェック
/// - 空文字列は無視
/// - 前後の空白をトリム
pub fn add_to_csv(csv: &str, item: &str) -> String {
    let mut items: Vec<String> = csv
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let trimmed = item.trim().to_string();
    if !trimmed.is_empty() && !items.iter().any(|i| i.eq_ignore_ascii_case(&trimmed)) {
        items.push(trimmed);
    }
    items.join(", ")
}

/// カンマ区切り文字列からインデックス指定で要素を削除する。
/// 範囲外インデックスは no-op。
pub fn remove_from_csv(csv: &str, idx: usize) -> String {
    let mut items: Vec<String> = csv
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if idx < items.len() {
        items.remove(idx);
    }
    items.join(", ")
}
