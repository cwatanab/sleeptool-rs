//! リソースメトリクスの履歴管理（固定長リングバッファ）。
//! ヒープアロケーションなしで直近30サンプルの時系列データを保持します。

pub const HISTORY_CAPACITY: usize = 30;

#[derive(Clone, Copy)]
pub struct MetricHistory {
    data: [f32; HISTORY_CAPACITY],
    head: usize,
    count: usize,
}

impl MetricHistory {
    pub const fn new() -> Self {
        Self {
            data: [0.0; HISTORY_CAPACITY],
            head: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, val: f32) {
        self.data[self.head] = val;
        self.head = (self.head + 1) % HISTORY_CAPACITY;
        if self.count < HISTORY_CAPACITY {
            self.count += 1;
        }
    }

    pub fn last(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            let last_idx = (self.head + HISTORY_CAPACITY - 1) % HISTORY_CAPACITY;
            self.data[last_idx]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn max(&self) -> f32 {
        let mut m: f32 = 0.0;
        for i in 0..self.count {
            if self.data[i] > m {
                m = self.data[i];
            }
        }
        m
    }

    /// GDI Polyline 用の座標配列を生成（時系列順）
    pub fn build_points(
        &self,
        rect_x: i32,
        rect_y: i32,
        width: i32,
        height: i32,
        max_val: f32,
    ) -> ([windows::Win32::Foundation::POINT; HISTORY_CAPACITY], usize) {
        use windows::Win32::Foundation::POINT;
        let mut pts = [POINT { x: 0, y: 0 }; HISTORY_CAPACITY];
        let n = self.count;
        if n < 2 {
            return (pts, 0);
        }

        let max_val = if max_val <= 0.0 { 100.0 } else { max_val };

        for i in 0..n {
            let idx = (self.head + HISTORY_CAPACITY - n + i) % HISTORY_CAPACITY;
            let val = self.data[idx].clamp(0.0, max_val);

            let x = rect_x + (i as i32 * width) / (n as i32 - 1);
            let y = rect_y + height - ((val / max_val) * height as f32) as i32;

            pts[i] = POINT { x, y };
        }
        (pts, n)
    }
}
