//! リソース監視ミニHUD（ウィジェット）モジュール。
//! Win32 Layered Window (GDI ダブルバッファリング) による超軽量・低負荷なフローティングHUD。

pub mod history;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleBitmap, CreateCompatibleDC, CreateFontW, CreatePen, CreatePolygonRgn,
    CreateRoundRectRgn, CreateSolidBrush, DeleteDC, DeleteObject, DrawTextW, FillRect, FillRgn,
    FrameRgn, GetDC, Polyline, ReleaseDC, SelectClipRgn, SelectObject, SetBkMode, SetTextColor,
    SetWindowRgn, BLENDFUNCTION, DT_CENTER, DT_NOCLIP, DT_RIGHT, DT_SINGLELINE, DT_VCENTER,
    FW_NORMAL, FW_SEMIBOLD, HDC, HFONT, HRGN, PS_DOT, PS_SOLID, TRANSPARENT, WINDING,
};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
    GetWindowLongPtrW, GetWindowRect, PostQuitMessage, RegisterClassW, SendMessageW,
    SetWindowLongPtrW, SetWindowPos, ShowWindow, UpdateLayeredWindow, CREATESTRUCTW,
    GWLP_USERDATA, HTCAPTION, MSG, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_SHOW,
    ULW_ALPHA, WNDCLASSW, WM_CREATE, WM_DESTROY, WM_EXITSIZEMOVE, WM_LBUTTONDOWN,
    WM_NCLBUTTONDOWN, WM_SETTINGCHANGE, WM_USER, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

use crate::platform::PerformanceSnapshot;
use crate::state::SharedState;
use history::MetricHistory;

pub const WM_WIDGET_UPDATE: u32 = WM_USER + 20;

// アイコン表示、タイトル行なし、スパークライン拡大版のジャストフィットサイズ
const WIDGET_WIDTH: i32 = 220;
const WIDGET_HEIGHT: i32 = 88;

#[inline(always)]
const fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

pub struct WidgetContext {
    pub state: SharedState,
    pub hwnd: HWND,
    pub cpu_history: MetricHistory,
    pub net_history: MetricHistory,
    pub disk_history: MetricHistory,
}

unsafe extern "system" fn widget_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let cs = &*(lparam.0 as *const CREATESTRUCTW);
            let ctx_ptr = cs.lpCreateParams as isize;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, ctx_ptr);
            let ctx = &mut *(ctx_ptr as *mut WidgetContext);
            ctx.hwnd = hwnd;
            return LRESULT(0);
        }
        WM_LBUTTONDOWN => {
            // ウィンドウのどこをドラッグしても移動できるようにする
            let _ = ReleaseCapture();
            let _ = SendMessageW(
                hwnd,
                WM_NCLBUTTONDOWN,
                WPARAM(HTCAPTION as usize),
                LPARAM(0),
            );
            return LRESULT(0);
        }
        WM_EXITSIZEMOVE => {
            // 移動終了時に位置を保存
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let ctx = &*(ctx_ptr as *mut WidgetContext);
                let mut rect = RECT::default();
                if GetWindowRect(hwnd, &mut rect).is_ok() {
                    let mut s = ctx.state.lock().unwrap();
                    let mut cfg = (*s.config).clone();
                    cfg.widget.x = rect.left;
                    cfg.widget.y = rect.top;
                    let _ = cfg.save(&s.config_path);
                    s.config = Arc::new(cfg);
                }
            }
            return LRESULT(0);
        }
        WM_WIDGET_UPDATE | WM_SETTINGCHANGE => {
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let ctx = &mut *(ctx_ptr as *mut WidgetContext);
                render_widget(ctx);
            }
            return LRESULT(0);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            return LRESULT(0);
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

pub unsafe fn render_widget(ctx: &mut WidgetContext) {
    let (alpha, cpu_threshold, net_threshold, disk_threshold) = {
        let s = ctx.state.lock().unwrap();
        let cpu_th = s.config.cpu.enabled.then_some(s.config.cpu.threshold as f32);
        let net_th = s.config.network.enabled.then_some((s.config.network.threshold / 1024.0) as f32);
        let disk_th = s.config.disk.write_enabled.then_some((s.config.disk.write_threshold / 1024.0) as f32);
        // アクリル風の半透明度 (デフォルト 210 = 約 82%) を反映
        let effective_alpha = s.config.widget.alpha.clamp(140, 255);
        (effective_alpha, cpu_th, net_th, disk_th)
    };

    let screen_dc = GetDC(HWND::default());
    let mem_dc = CreateCompatibleDC(screen_dc);
    let mem_bmp = CreateCompatibleBitmap(screen_dc, WIDGET_WIDTH, WIDGET_HEIGHT);
    let old_bmp = SelectObject(mem_dc, mem_bmp);

    let dark = crate::tray::theme::is_system_dark_mode();

    // テーマカラー設計（Fluent / Modern SaaS 階調）
    let background = if dark { rgb(20, 23, 28) } else { rgb(252, 252, 254) };
    let border_outer = if dark { rgb(50, 56, 66) } else { rgb(209, 213, 219) };
    let border_inner = if dark { rgb(32, 37, 45) } else { rgb(243, 244, 246) };

    let text_color = if dark { rgb(241, 245, 249) } else { rgb(17, 24, 39) };
    let alert_color = if dark { rgb(248, 113, 113) } else { rgb(220, 38, 38) };
    let alert_fill = if dark { rgb(52, 22, 25) } else { rgb(254, 226, 226) };

    let chart_background = if dark { rgb(14, 16, 20) } else { rgb(241, 245, 249) };
    let chart_border = if dark { rgb(38, 44, 53) } else { rgb(226, 232, 240) };
    let guide_color = if dark { rgb(55, 65, 81) } else { rgb(203, 213, 225) };

    // 1. エレガントな角丸カード背景（リソース消費ゼロ）
    let full_rect = RECT {
        left: 0,
        top: 0,
        right: WIDGET_WIDTH,
        bottom: WIDGET_HEIGHT,
    };
    let bg_brush = CreateSolidBrush(background);
    FillRect(mem_dc, &full_rect, bg_brush);
    let _ = DeleteObject(bg_brush);

    // 外枠スリムボーダー (1px)
    let card_rgn = CreateRoundRectRgn(0, 0, WIDGET_WIDTH, WIDGET_HEIGHT, 16, 16);
    let border_brush = CreateSolidBrush(border_outer);
    let _ = FrameRgn(mem_dc, card_rgn, border_brush, 1, 1);
    let _ = DeleteObject(border_brush);
    let _ = DeleteObject(card_rgn);

    // インナーハイライト (1px インセット)
    let inner_rgn = CreateRoundRectRgn(1, 1, WIDGET_WIDTH - 1, WIDGET_HEIGHT - 1, 14, 14);
    let inner_brush = CreateSolidBrush(border_inner);
    let _ = FrameRgn(mem_dc, inner_rgn, inner_brush, 1, 1);
    let _ = DeleteObject(inner_brush);
    let _ = DeleteObject(inner_rgn);

    SetBkMode(mem_dc, TRANSPARENT);

    // ClearType 高品位 Segoe UI フォントの作成 (Segoe UI + Segoe UI Emoji)
    let font_metric = CreateFontW(
        -12, 0, 0, 0, FW_SEMIBOLD.0 as i32, 0, 0, 0, 1, 0, 0, 5, 0, w!("Segoe UI"),
    );
    let font_icon = CreateFontW(
        -12, 0, 0, 0, FW_NORMAL.0 as i32, 0, 0, 0, 1, 0, 0, 5, 0, w!("Segoe UI Emoji"),
    );
    let old_font = SelectObject(mem_dc, font_metric);

    // 行間余白をゆったり取って3行を配置 (y: 8, 34, 60, 余白 8px)
    // 行 1: CPU (0〜100%) - アイコン ⚡ (アンバーゴールド系)
    let cur_cpu = ctx.cpu_history.last();
    let cpu_str = format!("{:.1}%", cur_cpu);
    let is_cpu_alert = threshold_reached(&ctx.cpu_history, cpu_threshold);
    let (cpu_color, cpu_fill_color, cpu_icon_color, cpu_text_color) = if is_cpu_alert {
        (alert_color, alert_fill, alert_color, alert_color)
    } else {
        (
            if dark { rgb(245, 158, 11) } else { rgb(217, 119, 6) },
            if dark { rgb(46, 34, 16) } else { rgb(254, 243, 199) },
            if dark { rgb(245, 158, 11) } else { rgb(217, 119, 6) },
            text_color,
        )
    };
    draw_metric_row(
        mem_dc,
        "⚡",
        font_icon,
        font_metric,
        cpu_str,
        &ctx.cpu_history,
        8,
        100.0,
        cpu_threshold,
        cpu_color,
        cpu_fill_color,
        cpu_icon_color,
        cpu_text_color,
        chart_background,
        chart_border,
        guide_color,
    );

    // 行 2: ネット (KB/s, 動的オートスケーリング) - アイコン 🌐 (エメラルドミント系)
    let cur_net_kb = ctx.net_history.last();
    let net_scale = ctx.net_history.max().max(net_threshold.unwrap_or(0.0) * 1.2).max(20.0);
    let net_str = format_rate(cur_net_kb);
    let is_net_alert = threshold_reached(&ctx.net_history, net_threshold);
    let (net_color, net_fill_color, net_icon_color, net_text_color) = if is_net_alert {
        (alert_color, alert_fill, alert_color, alert_color)
    } else {
        (
            if dark { rgb(16, 185, 129) } else { rgb(5, 150, 105) },
            if dark { rgb(16, 38, 28) } else { rgb(209, 250, 229) },
            if dark { rgb(16, 185, 129) } else { rgb(5, 150, 105) },
            text_color,
        )
    };
    draw_metric_row(
        mem_dc,
        "🌐",
        font_icon,
        font_metric,
        net_str,
        &ctx.net_history,
        34,
        net_scale,
        net_threshold,
        net_color,
        net_fill_color,
        net_icon_color,
        net_text_color,
        chart_background,
        chart_border,
        guide_color,
    );

    // 行 3: ディスク (KB/s, 動的オートスケーリング) - アイコン 💾 (スカイシアン系)
    let cur_disk_kb = ctx.disk_history.last();
    let disk_scale = ctx.disk_history.max().max(disk_threshold.unwrap_or(0.0) * 1.2).max(50.0);
    let disk_str = format_rate(cur_disk_kb);
    let is_disk_alert = threshold_reached(&ctx.disk_history, disk_threshold);
    let (disk_color, disk_fill_color, disk_icon_color, disk_text_color) = if is_disk_alert {
        (alert_color, alert_fill, alert_color, alert_color)
    } else {
        (
            if dark { rgb(14, 165, 233) } else { rgb(2, 132, 199) },
            if dark { rgb(14, 34, 46) } else { rgb(224, 242, 254) },
            if dark { rgb(14, 165, 233) } else { rgb(2, 132, 199) },
            text_color,
        )
    };
    draw_metric_row(
        mem_dc,
        "💾",
        font_icon,
        font_metric,
        disk_str,
        &ctx.disk_history,
        60,
        disk_scale,
        disk_threshold,
        disk_color,
        disk_fill_color,
        disk_icon_color,
        disk_text_color,
        chart_background,
        chart_border,
        guide_color,
    );

    // 後始末
    SelectObject(mem_dc, old_font);
    let _ = DeleteObject(font_metric);
    let _ = DeleteObject(font_icon);

    // UpdateLayeredWindow による一括転送
    let blend = BLENDFUNCTION {
        BlendOp: windows::Win32::Graphics::Gdi::AC_SRC_OVER as u8,
        BlendFlags: 0,
        SourceConstantAlpha: alpha,
        AlphaFormat: 0,
    };

    let pt_src = POINT { x: 0, y: 0 };
    let size = SIZE {
        cx: WIDGET_WIDTH,
        cy: WIDGET_HEIGHT,
    };

    let _ = UpdateLayeredWindow(
        ctx.hwnd,
        screen_dc,
        None,
        Some(&size),
        mem_dc,
        Some(&pt_src),
        COLORREF(0),
        Some(&blend),
        ULW_ALPHA,
    );

    SelectObject(mem_dc, old_bmp);
    let _ = DeleteObject(mem_bmp);
    let _ = DeleteDC(mem_dc);
    ReleaseDC(HWND::default(), screen_dc);
}

fn format_rate(kb: f32) -> String {
    if kb >= 1024.0 {
        format!("{:.1} MB/s", kb / 1024.0)
    } else {
        format!("{:.0} KB/s", kb)
    }
}

fn threshold_reached(history: &MetricHistory, threshold: Option<f32>) -> bool {
    !history.is_empty() && threshold.is_some_and(|threshold| history.last() >= threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_alert_requires_enabled_threshold_and_sample() {
        let mut history = MetricHistory::new();
        assert!(!threshold_reached(&history, Some(0.0)));
        history.push(9.0);
        assert!(!threshold_reached(&history, Some(10.0)));
        assert!(!threshold_reached(&history, None));
        history.push(10.0);
        assert!(threshold_reached(&history, Some(10.0)));
        history.push(11.0);
        assert!(threshold_reached(&history, Some(10.0)));
        history.push(9.0);
        assert!(!threshold_reached(&history, Some(10.0)));
    }
}

unsafe fn draw_metric_row(
    hdc: HDC,
    icon: &str,
    font_icon: HFONT,
    font_metric: HFONT,
    val_str: String,
    history: &MetricHistory,
    y: i32,
    max_scale: f32,
    threshold: Option<f32>,
    color: COLORREF,
    fill_color: COLORREF,
    icon_color: COLORREF,
    text_color: COLORREF,
    chart_bg_color: COLORREF,
    chart_border_color: COLORREF,
    guide_color: COLORREF,
) {
    // 1. アイコン (⚡ / 🌐 / 💾)
    SelectObject(hdc, font_icon);
    SetTextColor(hdc, icon_color);
    let mut icon_w: Vec<u16> = icon.encode_utf16().collect();
    let mut icon_rect = RECT {
        left: 8,
        top: y,
        right: 26,
        bottom: y + 20,
    };
    DrawTextW(
        hdc,
        &mut icon_w,
        &mut icon_rect,
        DT_CENTER | DT_SINGLELINE | DT_VCENTER,
    );

    // 2. チャート背景枠 (スパークライン: 118px × 18px、角丸クリッピングで上品に)
    let chart_x = 28;
    let chart_y = y + 1;
    let chart_w = 118;
    let chart_h = 18;

    let chart_rgn = CreateRoundRectRgn(chart_x, chart_y, chart_x + chart_w + 1, chart_y + chart_h + 1, 6, 6);
    let chart_bg = CreateSolidBrush(chart_bg_color);
    let _ = FillRgn(hdc, chart_rgn, chart_bg);
    let _ = DeleteObject(chart_bg);

    // 微細なチャート枠線
    let chart_border_brush = CreateSolidBrush(chart_border_color);
    let _ = FrameRgn(hdc, chart_rgn, chart_border_brush, 1, 1);
    let _ = DeleteObject(chart_border_brush);

    // チャート領域に描画をクリッピング
    SelectClipRgn(hdc, chart_rgn);

    // しきい値ガイドライン（設定されている場合、控えめな点線で表示）
    if let Some(th) = threshold {
        if th > 0.0 && th <= max_scale {
            let th_y = chart_y + chart_h - 1 - ((th / max_scale) * (chart_h - 2) as f32).round() as i32;
            let guide_pen = CreatePen(PS_DOT, 1, guide_color);
            let old_pen = SelectObject(hdc, guide_pen);
            let _ = windows::Win32::Graphics::Gdi::MoveToEx(hdc, chart_x, th_y, None);
            let _ = windows::Win32::Graphics::Gdi::LineTo(hdc, chart_x + chart_w, th_y);
            SelectObject(hdc, old_pen);
            let _ = DeleteObject(guide_pen);
        }
    }

    // 3. チャート折れ線 & エリアフィル (モダンエリアチャート表現)
    let (pts, count) = history.build_points(chart_x + 1, chart_y + 1, chart_w - 2, chart_h - 2, max_scale);
    if count >= 2 {
        // 折れ線の下部に上品なエリア塗りつぶし（スタック配列・追加コストゼロ）
        let mut poly_pts = [POINT { x: 0, y: 0 }; history::HISTORY_CAPACITY + 2];
        poly_pts[..count].copy_from_slice(&pts[..count]);
        poly_pts[count] = POINT { x: pts[count - 1].x, y: chart_y + chart_h - 1 };
        poly_pts[count + 1] = POINT { x: pts[0].x, y: chart_y + chart_h - 1 };

        let area_rgn = CreatePolygonRgn(&poly_pts[..count + 2], WINDING);
        let area_brush = CreateSolidBrush(fill_color);
        let _ = FillRgn(hdc, area_rgn, area_brush);
        let _ = DeleteObject(area_brush);
        let _ = DeleteObject(area_rgn);

        // 1px のシャープなメイン折れ線
        let pen = CreatePen(PS_SOLID, 1, color);
        let old_pen = SelectObject(hdc, pen);
        let _ = Polyline(hdc, &pts[..count]);
        SelectObject(hdc, old_pen);
        let _ = DeleteObject(pen);

        // 最新値（右端）のポイントに小さな丸ドットを描画
        let last_pt = pts[count - 1];
        let dot_brush = CreateSolidBrush(color);
        let dot_rect = RECT {
            left: last_pt.x - 1,
            top: last_pt.y - 1,
            right: last_pt.x + 2,
            bottom: last_pt.y + 2,
        };
        FillRect(hdc, &dot_rect, dot_brush);
        let _ = DeleteObject(dot_brush);
    }

    // クリップ領域を解除してリージョン破棄
    SelectClipRgn(hdc, HRGN::default());
    let _ = DeleteObject(chart_rgn);

    // 4. 現在値の表示
    SelectObject(hdc, font_metric);
    SetTextColor(hdc, text_color);

    let mut val_w: Vec<u16> = val_str.encode_utf16().collect();
    let mut val_rect = RECT {
        left: chart_x + chart_w + 4,
        top: y,
        right: WIDGET_WIDTH - 8,
        bottom: y + 20,
    };
    DrawTextW(
        hdc,
        &mut val_w,
        &mut val_rect,
        DT_RIGHT | DT_SINGLELINE | DT_VCENTER | DT_NOCLIP,
    );
}

/// ウィジェットウィンドウを作成してスレッドを起動
pub fn spawn_widget(
    state: SharedState,
    running: Arc<AtomicBool>,
) -> anyhow::Result<std::thread::JoinHandle<()>> {
    let handle = std::thread::spawn(move || unsafe {
        let class_name: Vec<u16> = "SleepToolWidgetClass\0".encode_utf16().collect();
        let hinstance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)
            .unwrap_or_default();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(widget_wnd_proc),
            hInstance: windows::Win32::Foundation::HINSTANCE(hinstance.0),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        let (init_x, init_y, enabled) = {
            let s = state.lock().unwrap();
            (s.config.widget.x, s.config.widget.y, s.config.widget.enabled)
        };

        let ctx = Box::new(WidgetContext {
            state: state.clone(),
            hwnd: HWND::default(),
            cpu_history: MetricHistory::new(),
            net_history: MetricHistory::new(),
            disk_history: MetricHistory::new(),
        });
        let ctx_ptr = Box::into_raw(ctx);

        let window_name: Vec<u16> = "SleepTool HUD\0".encode_utf16().collect();
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
            PCWSTR(class_name.as_ptr()),
            PCWSTR(window_name.as_ptr()),
            WS_POPUP,
            init_x,
            init_y,
            WIDGET_WIDTH,
            WIDGET_HEIGHT,
            HWND::default(),
            windows::Win32::UI::WindowsAndMessaging::HMENU::default(),
            windows::Win32::Foundation::HINSTANCE(hinstance.0),
            Some(ctx_ptr as *const std::ffi::c_void),
        );

        if let Ok(hwnd) = hwnd {
            // OS レベルでウィンドウを角丸にクリップ（四隅の黒塗りアーティファクトを防止し、ヒットテストも正確に）
            let rgn = CreateRoundRectRgn(0, 0, WIDGET_WIDTH + 1, WIDGET_HEIGHT + 1, 16, 16);
            let _ = SetWindowRgn(hwnd, rgn, true);

            {
                let mut s = state.lock().unwrap();
                s.widget_hwnd = Some(hwnd.0 as isize);
            }

            render_widget(&mut *(ctx_ptr as *mut WidgetContext));

            if enabled {
                let _ = ShowWindow(hwnd, SW_SHOW);
            } else {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }

            let mut msg = MSG::default();
            while running.load(Ordering::Relaxed) && GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = DestroyWindow(hwnd);
            let _ctx = Box::from_raw(ctx_ptr);
        }
    });

    Ok(handle)
}

/// ウィジェットの表示/非表示を切り替え
pub unsafe fn toggle_widget(state: &SharedState) {
    let (hwnd_val, enabled) = {
        let mut s = state.lock().unwrap();
        let mut cfg = (*s.config).clone();
        cfg.widget.enabled = !cfg.widget.enabled;
        let _ = cfg.save(&s.config_path);
        s.config = Arc::new(cfg);
        (s.widget_hwnd, s.config.widget.enabled)
    };

    if let Some(hwnd_isize) = hwnd_val {
        let hwnd = HWND(hwnd_isize as *mut std::ffi::c_void);
        if enabled {
            let _ = ShowWindow(hwnd, SW_SHOW);
            let _ = SetWindowPos(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        } else {
            let _ = ShowWindow(hwnd, SW_HIDE);
        }
    }
}

/// メトリクス更新時にウィジェットに通知
pub unsafe fn notify_widget_update(state: &SharedState, perf: &PerformanceSnapshot) {
    let (hwnd_val, enabled) = {
        let s = state.lock().unwrap();
        (s.widget_hwnd, s.config.widget.enabled)
    };

    if enabled {
        if let Some(hwnd_isize) = hwnd_val {
            let hwnd = HWND(hwnd_isize as *mut std::ffi::c_void);
            let ctx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
            if ctx_ptr != 0 {
                let ctx = &mut *(ctx_ptr as *mut WidgetContext);
                ctx.cpu_history.push(perf.cpu_percent as f32);
                ctx.net_history.push((perf.network_bytes_per_sec / 1024.0) as f32);
                ctx.disk_history.push((perf.disk_write_bytes_per_sec / 1024.0) as f32);
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    hwnd,
                    WM_WIDGET_UPDATE,
                    WPARAM(0),
                    LPARAM(0),
                );
            }
        }
    }
}
