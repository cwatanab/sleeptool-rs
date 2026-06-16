//! トレイアイコン画像の管理。
//!
//! ビルド時に `build.rs` で生成された `icons_data.rs` から RGBA データを
//! 取り出し、Win32 の `HICON` ハンドルに変換する。
//!
//! データ形式:
//! - 1 バイト目 `0`: 非圧縮。残りはそのまま RGBA 列。
//! - 1 バイト目 `1`: パレット圧縮。2 バイト目がパレット長-1。

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateDIBSection, DeleteObject, GetDC, ReleaseDC, BITMAPINFO, BITMAPINFOHEADER,
    DIB_RGB_COLORS,
};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconIndirect, HICON, ICONINFO};

use crate::monitors::InhibitFactor;

include!(concat!(env!("OUT_DIR"), "/icons_data.rs"));

/// 内部表現（パレット圧縮 / 非圧縮）を RGBA バイト列に展開。
pub fn decompact(data: &[u8]) -> Vec<u8> {
    if data[0] == 0 {
        data[1..].to_vec()
    } else {
        let palette_size = data[1] as usize + 1;
        let palette_start = 2;
        let indices_start = palette_start + palette_size * 4;
        let pixel_count = data.len() - indices_start;
        let mut rgba = vec![0u8; pixel_count * 4];
        for i in 0..pixel_count {
            let idx = data[indices_start + i] as usize;
            let p = palette_start + idx * 4;
            rgba[i * 4] = data[p];
            rgba[i * 4 + 1] = data[p + 1];
            rgba[i * 4 + 2] = data[p + 2];
            rgba[i * 4 + 3] = data[p + 3];
        }
        rgba
    }
}

/// RGBA 列から `HICON` を作成する。`width x height` の 32bit ビットマップ。
///
/// # Safety
///
/// GDI API を呼び出す。
pub unsafe fn create_hicon(rgba: &[u8], width: i32, height: i32) -> windows::core::Result<HICON> {
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let hdc = GetDC(HWND::default());
    let hcolor = CreateDIBSection(hdc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0)?;
    ReleaseDC(HWND::default(), hdc);

    let bits_slice = std::slice::from_raw_parts_mut(bits as *mut u8, (width * height * 4) as usize);
    for i in 0..(width * height) as usize {
        let r = rgba[i * 4];
        let g = rgba[i * 4 + 1];
        let b = rgba[i * 4 + 2];
        let a = rgba[i * 4 + 3];
        bits_slice[i * 4] = b;
        bits_slice[i * 4 + 1] = g;
        bits_slice[i * 4 + 2] = r;
        bits_slice[i * 4 + 3] = a;
    }

    let mask_bits = vec![0u8; (width * height / 8) as usize];
    let hmask = CreateBitmap(width, height, 1, 1, Some(mask_bits.as_ptr() as *const _));

    let mut icon_info = ICONINFO {
        fIcon: true.into(),
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hmask,
        hbmColor: hcolor,
    };

    let hicon = CreateIconIndirect(&mut icon_info)?;

    let _ = DeleteObject(hcolor);
    let _ = DeleteObject(hmask);

    Ok(hicon)
}

/// アイコン名（"default", "cpu", "network", "disk", "sound", "process", "paused"）
/// に対応する `HICON` を作成する。
pub fn load_handle(name: &str) -> anyhow::Result<HICON> {
    let data: &[u8] = match name {
        "default" => ICON_DEFAULT,
        "cpu" => ICON_CPU,
        "network" => ICON_NETWORK,
        "disk" => ICON_DISK,
        "sound" => ICON_SOUND,
        "process" => ICON_PROCESS,
        "paused" => ICON_PAUSED,
        _ => ICON_DEFAULT,
    };
    let rgba = decompact(data);
    unsafe {
        create_hicon(&rgba, 32, 32)
            .map_err(|e| anyhow::anyhow!("GDI error: {:?}", e))
    }
}

/// 全状態のアイコンをまとめて保持。
pub struct IconSet {
    pub default: HICON,
    pub paused: HICON,
    pub cpu: HICON,
    pub network: HICON,
    pub disk: HICON,
    pub sound: HICON,
    pub process: HICON,
}

impl IconSet {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            default: load_handle("default")?,
            paused: load_handle("paused")?,
            cpu: load_handle("cpu")?,
            network: load_handle("network")?,
            disk: load_handle("disk")?,
            sound: load_handle("sound")?,
            process: load_handle("process")?,
        })
    }

    /// 現在の inhibit factor / ポーズ状態から表示すべきアイコンを選択する。
    pub fn pick(&self, factor: Option<InhibitFactor>, paused: bool) -> HICON {
        if paused {
            return self.paused;
        }
        match factor {
            Some(InhibitFactor::Process) => self.process,
            Some(InhibitFactor::Sound) => self.sound,
            Some(InhibitFactor::Cpu) => self.cpu,
            Some(InhibitFactor::Network) => self.network,
            Some(InhibitFactor::DiskRead) | Some(InhibitFactor::DiskWrite) => self.disk,
            Some(InhibitFactor::Input) | None => self.default,
        }
    }
}

impl Drop for IconSet {
    fn drop(&mut self) {
        unsafe {
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.default);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.paused);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.cpu);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.network);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.disk);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.sound);
            let _ = windows::Win32::UI::WindowsAndMessaging::DestroyIcon(self.process);
        }
    }
}

/// `NOTIFYICONDATAW` のツールチップテキストを設定する。最大 127 文字。
pub fn set_tooltip(nid: &mut windows::Win32::UI::Shell::NOTIFYICONDATAW, text: &str) {
    let mut tip = [0u16; 128];
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let len = utf16.len().min(127);
    tip[..len].copy_from_slice(&utf16[..len]);
    nid.szTip = tip;
}

/// PCWSTR のヘルパ（再エクスポート）
pub fn pcwstr(s: &[u16]) -> PCWSTR {
    PCWSTR(s.as_ptr())
}
