use std::f32::consts::{PI, SQRT_2, TAU};

use scissor::{glam::*, *};

fn main() {
    let cfg = Config { resolution: 0.1 };

    let shape = Parametric::new(|x| Vec2::new(x.sin(), -x.cos()), 0.0..TAU)
        .complete()
        .map(|v| {
            let f = PI / 4.0;

            let p = f.sin();

            if v.y < -p {
                v.y = v.x.abs() / f.tan() - SQRT_2;
            }
        })
        .hole(Circle::new(0.4))
        .split(
            |shape| shape.fill([0.1, 0.2, 0.6, 1.0]),
            |shape| shape.outline(0.1).fill([0.0, 0.0, 0.0, 1.0]),
        )
        .combine();

    let t = std::time::Instant::now();
    shape.generate(&cfg, ());

    println!("{:?}", std::time::Instant::now() - t);
}
