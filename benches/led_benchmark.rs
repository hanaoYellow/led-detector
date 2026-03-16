use criterion::{Criterion, criterion_group, criterion_main};
use led_detector::{HsvRange, detect_from_image_parallel, detect_from_image_serial};

fn benchmark_serial(c: &mut Criterion) {
    let normal_range = HsvRange {
        h_min: 90.0,
        h_max: 160.0,
        s_min: 0.3,
        s_max: 1.0,
        v_min: 0.3,
        v_max: 1.0,
    };

    c.bench_function("serial", |b| {
        b.iter(|| {
            detect_from_image_serial("./images/green.png", &normal_range);
        })
    });
}

fn benchmark_parallel(c: &mut Criterion) {
    let normal_range = HsvRange {
        h_min: 90.0,
        h_max: 160.0,
        s_min: 0.3,
        s_max: 1.0,
        v_min: 0.3,
        v_max: 1.0,
    };

    c.bench_function("parallel", |b| {
        b.iter(|| {
            detect_from_image_parallel("./images/green.png", &normal_range);
        })
    });
}

criterion_group!(benches, benchmark_serial, benchmark_parallel);
criterion_main!(benches);
