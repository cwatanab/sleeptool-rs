//! COM 経由で再生デバイスのピークメーターおよびアクティブなオーディオセッションを監視する。

use std::cell::Cell;

use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{
    eConsole, eMultimedia, eRender, AudioSessionStateActive,
    IAudioSessionManager2, IMMDevice, IMMDeviceEnumerator,
    MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};

use crate::error::Result;

thread_local! {
    static COM_INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

fn ensure_com_initialized() {
    COM_INITIALIZED.with(|init| {
        if !init.get() {
            unsafe { let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED); }
            init.set(true);
        }
    });
}

/// 指定したデバイスのピーク値を取得する。
unsafe fn read_device_peak(device: &IMMDevice) -> Option<f32> {
    let meter: IAudioMeterInformation = device.Activate(CLSCTX_ALL, None).ok()?;
    meter.GetPeakValue().ok()
}

/// 指定したデバイス内にアクティブ（再生中）なオーディオセッションが存在するか調べる。
unsafe fn has_active_session(device: &IMMDevice) -> bool {
    let Ok(mgr) = device.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None) else {
        return false;
    };
    let Ok(enumerator) = mgr.GetSessionEnumerator() else {
        return false;
    };
    let Ok(count) = enumerator.GetCount() else {
        return false;
    };

    for i in 0..count {
        if let Ok(control) = enumerator.GetSession(i) {
            if let Ok(state) = control.GetState() {
                if state == AudioSessionStateActive {
                    return true;
                }
            }
        }
    }
    false
}

pub fn current_peak() -> Result<f64> {
    ensure_com_initialized();

    let mut max_peak = 0.0f32;
    let mut any_session_active = false;

    unsafe {
        let Ok(enumerator) = CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL) else {
            return Ok(0.0);
        };

        // 1. 全てのアクティブな再生デバイスを列挙してチェック
        if let Ok(collection) = enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE) {
            if let Ok(count) = collection.GetCount() {
                for i in 0..count {
                    if let Ok(device) = collection.Item(i) {
                        if let Some(peak) = read_device_peak(&device) {
                            if peak > max_peak {
                                max_peak = peak;
                            }
                        }
                        if !any_session_active && has_active_session(&device) {
                            any_session_active = true;
                        }
                    }
                }
            }
        }

        // 2. もしデバイス列挙で何も取れなかった場合のフォールバック: eConsole / eMultimedia
        if max_peak == 0.0 && !any_session_active {
            for role in [eConsole, eMultimedia] {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, role) {
                    if let Some(peak) = read_device_peak(&device) {
                        if peak > max_peak {
                            max_peak = peak;
                        }
                    }
                    if !any_session_active && has_active_session(&device) {
                        any_session_active = true;
                    }
                }
            }
        }
    }

    // アプリが再生中 (AudioSessionStateActive) なら、音が小さくてもスリープを阻止するため 1.0 を返す
    if any_session_active {
        return Ok(1.0);
    }

    Ok(max_peak as f64)
}
