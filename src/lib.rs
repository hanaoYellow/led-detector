use rayon::prelude::*;

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

pub struct HsvRange {
    pub h_min: f32,
    pub h_max: f32,
    pub s_min: f32,
    pub s_max: f32,
    pub v_min: f32,
    pub v_max: f32,
}

pub trait ToHsv {
    fn to_hsv(&self) -> Hsv;
}

pub trait LedDetector: Send + Sync {
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

// 直列処理版
pub fn detect_from_image_serial(path: &str, detector: &dyn LedDetector) -> f32 {
    let img = image::open(path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();
    let total = (width * height) as usize;

    let abnormal_count = img
        .pixels()
        .filter(|pixel| {
            let rgb = Rgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };
            let hsv = rgb.to_hsv();
            !detector.is_normal(&hsv)
        })
        .count();

    abnormal_count as f32 / total as f32 * 100.0
}

// 並列処理版
pub fn detect_from_image_parallel(path: &str, detector: &dyn LedDetector) -> f32 {
    let img = image::open(path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();
    let total = (width * height) as usize;

    let abnormal_count = img
        .pixels()
        .par_bridge()
        .filter(|pixel| {
            let rgb = Rgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            };
            let hsv = rgb.to_hsv();
            !detector.is_normal(&hsv)
        })
        .count();

    abnormal_count as f32 / total as f32 * 100.0
}
