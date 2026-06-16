use sleeptool_rs::monitors::threshold::{evaluate, ThresholdState};

#[test]
fn inactive_below_threshold_first_sample() {
    let mut s = ThresholdState::default();
    assert!(!evaluate(&mut s, 10.0, 50.0, 60));
}

#[test]
fn active_at_or_above_threshold() {
    let mut s = ThresholdState::default();
    assert!(evaluate(&mut s, 50.0, 50.0, 60));
}

#[test]
fn above_threshold() {
    let mut s = ThresholdState::default();
    assert!(evaluate(&mut s, 75.0, 50.0, 60));
}

#[test]
fn holds_during_delay_after_active_pulse() {
    let mut s = ThresholdState::default();
    assert!(evaluate(&mut s, 100.0, 50.0, 60));
    // 値がしきい値未満に戻っても、delay_seconds 内なら inhibit
    assert!(evaluate(&mut s, 0.0, 50.0, 60));
}

#[test]
fn stops_inhibiting_after_delay_elapses() {
    let mut s = ThresholdState::default();
    assert!(evaluate(&mut s, 100.0, 50.0, 0));
    // delay_seconds = 0 なので、2回目で必ず非 inhibit
    assert!(!evaluate(&mut s, 0.0, 50.0, 0));
}

#[test]
fn fresh_state_with_no_active_history_does_not_inhibit() {
    let mut s = ThresholdState::default();
    // 1回目に非アクティブ → last_active はセットされない
    assert!(!evaluate(&mut s, 0.0, 50.0, 60));
    // 2回目も非アクティブ → 依然として inhibit しない
    assert!(!evaluate(&mut s, 0.0, 50.0, 60));
}

#[test]
fn holds_only_for_exact_delay_seconds() {
    // delay_seconds=1 だと、as_secs() の整数秒丸めで 1秒未満なら inhibit
    let mut s = ThresholdState::default();
    assert!(evaluate(&mut s, 100.0, 50.0, 1));
    // しきい値未満で 2 回目、as_secs() < 1 → 整数秒差が 0 なら inhibit
    let inhibit = evaluate(&mut s, 0.0, 50.0, 1);
    // 実装上、as_secs() < 1 は as_secs() == 0 のとき成立
    // テストは短時間内に連続呼ぶので inhibit する
    assert!(inhibit);
}

#[test]
fn state_default_has_no_active_history() {
    let s = ThresholdState::default();
    assert!(s.last_active().is_none());
}
