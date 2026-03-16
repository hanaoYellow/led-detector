use rayon::prelude::*;
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

struct HsvRange {
    h_min: f32,
    h_max: f32,
    s_min: f32,
    s_max: f32,
    v_min: f32,
    v_max: f32,
}

trait ToHsv {
    fn to_hsv(&self) -> Hsv;
}

trait LedDetector: Send + Sync {
    fn is_normal(&self, hsv: &Hsv) -> bool;
}

impl ToHsv for Rgb {
    fn to_hsv(&self) -> Hsv {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        Hsv { h, s, v }
    }
}

impl LedDetector for HsvRange {
    fn is_normal(&self, hsv: &Hsv) -> bool {
        hsv.h >= self.h_min
            && hsv.h <= self.h_max
            && hsv.s >= self.s_min
            && hsv.s <= self.s_max
            && hsv.v >= self.v_min
            && hsv.v <= self.v_max
    }
}

fn main() {
    let red = Rgb { r: 255, g: 0, b: 0 };
    let hsv = red.to_hsv();
    println!("red H: {:.2}, S: {:.2}, V: {:.2}", hsv.h, hsv.s, hsv.v);

    let green = Rgb { r: 0, g: 255, b: 0 };
    let hsv = green.to_hsv();
    println!("green H: {:.2}, S: {:.2}, V: {:.2}", hsv.h, hsv.s, hsv.v);

    let normal_range = HsvRange {
        h_min: 90.0,
        h_max: 170.0,
        s_min: 0.1,
        s_max: 1.0,
        v_min: 0.3,
        v_max: 1.0,
    };

    let green = Rgb { r: 0, g: 255, b: 0 };
    let hsv = green.to_hsv();
    println!(
        "緑LED: {}",
        if normal_range.is_normal(&hsv) {
            "正常"
        } else {
            "異常"
        }
    );

    let red = Rgb { r: 255, g: 0, b: 0 };
    let hsv = red.to_hsv();
    println!(
        "赤LED: {}",
        if normal_range.is_normal(&hsv) {
            "正常"
        } else {
            "異常"
        }
    );

    detect_from_image("./images/green.png", &normal_range);
    detect_from_image("./images/orange.png", &normal_range);
    detect_from_image("./images/red.png", &normal_range);
}

fn detect_from_image(path: &str, detector: &dyn LedDetector) {
    let img = image::open(path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();

    let abnormal_count = img
        .pixels()
        .par_bridge() // 並列処理
        .filter(|pixel| {
            let rgb = Rgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };
            let hsv = rgb.to_hsv();
            !detector.is_normal(&hsv)
        })
        .count(); // usizeのまま

    let total = (width * height) as usize; // usizeに合わせる
    let abnormal_ratio = abnormal_count as f32 / total as f32 * 100.0;

    println!("ファイル: {}", path);
    println!("異常ピクセル率: {:.1}%", abnormal_ratio);
    println!(
        "判定: {}",
        if abnormal_ratio > 50.0 {
            "異常あり"
        } else {
            "正常"
        }
    );
}
