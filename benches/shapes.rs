use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scissor::{glam::*, *};

fn triangulate() {
    let config = Config { resolution: 0.05 };

    Circle::new(2.0).fill([0.0; 4]).generate(&config, ());
}

fn outline() {
    let config = Config { resolution: 0.05 };

    Circle::new(2.0).outline(0.3).generate(&config, ());
}

fn intersections() {
    let config = Config { resolution: 0.05 };

    Parametric::new(|x| Vec2::new(x, x.sin()), -2.0..2.0)
        .thicken(0.4)
        .generate(&config, ())
        .verify();
}

fn combine() {
    let config = Config { resolution: 0.05 };

    Circle::new(2.0)
        .map(|_| {})
        .split(|s| s.fill([1.0; 4]), |s| s.outline(0.4).fill([0.0; 4]))
        .combine()
        .generate(&config, ());
}

fn shapes(c: &mut Criterion) {
    c.bench_function("triangulate", |b| b.iter(|| triangulate()));
    c.bench_function("outline", |b| b.iter(|| outline()));
    c.bench_function("intersections", |b| b.iter(|| intersections()));
    c.bench_function("combine", |b| b.iter(|| combine()));
}

criterion_group!(benches, shapes);
criterion_main!(benches);
