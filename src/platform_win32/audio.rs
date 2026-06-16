//! COM 経由で再生デバイスのピークメーターにアクセスする。
//!
//! サウンドカード出力のピーク値（0.0..=1.0）をミリ秒以下で取得する。
//! ループバック録音より遥かに低コスト。

use std::cell::Cell;

use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{
    eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};

use crate::error::Result;

thread_local! {
    static COM_INITIALIZED: Cell<bool> = Cell::new(false);
}

fn ensure_com_initialized() {
    COM_INITIALIZED.with(|init| {
        if !init.get() {
            unsafe { let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED); }
            init.set(true);
        }
    });
}

/// 現在の出力音量ピーク (0.0..=1.0) を取得する。
/// 取得に失敗した場合は `Ok(0.0)`（無音扱い）。
pub fn current_peak() -> Result<f64> {
    ensure_com_initialized();
    unsafe {
        let result = (|| -> windows::core::Result<f64> {
            let enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let device =
                enumerator.GetDefaultAudioEndpoint(eRender, windows::Win32::Media::Audio::eConsole)?;
            let meter: IAudioMeterInformation = device.Activate(CLSCTX_ALL, None)?;
            let peak = meter.GetPeakValue()?;
            Ok(peak as f64)
        })();
        match result {
            Ok(val) => Ok(val),
            Err(_) => Ok(0.0),
        }
    }
}
