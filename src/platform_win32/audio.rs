//! COM 経由で再生デバイスのピークメーターにアクセスする。

use std::cell::Cell;

use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};

use crate::error::Result;

thread_local! {
    static COM_INITIALIZED: Cell<bool> = const { Cell::new(false) };
    static METER: std::cell::RefCell<Option<IAudioMeterInformation>> = const { std::cell::RefCell::new(None) };
}

fn ensure_com_initialized() {
    COM_INITIALIZED.with(|init| {
        if !init.get() {
            unsafe { let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED); }
            init.set(true);
        }
    });
}

fn get_meter() -> Option<IAudioMeterInformation> {
    METER.with(|cell| {
        if let Some(ref m) = *cell.borrow() { return Some(m.clone()); }
        let result: Option<IAudioMeterInformation> = (|| {
            let enumerator: IMMDeviceEnumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()? };
            let device = unsafe { enumerator.GetDefaultAudioEndpoint(eRender, windows::Win32::Media::Audio::eConsole).ok()? };
            unsafe { device.Activate(CLSCTX_ALL, None).ok() }
        })();
        if let Some(ref m) = result { *cell.borrow_mut() = Some(m.clone()); }
        result
    })
}

pub fn current_peak() -> Result<f64> {
    ensure_com_initialized();
    match get_meter() {
        Some(meter) => match unsafe { meter.GetPeakValue() } {
            Ok(v) => Ok(v as f64),
            Err(_) => { METER.with(|c| *c.borrow_mut() = None); Ok(0.0) }
        },
        None => Ok(0.0),
    }
}
