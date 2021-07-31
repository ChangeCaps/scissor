use std::f32::consts::{PI, SQRT_2};

use scissor::*;

fn main() {
    let cfg = Config { resolution: 0.05 };

    let shape = Circle::new(1.0)
        .map(|v| {
            if v.y < -(PI / 4.0).sin() {
                v.y = v.x.abs() - SQRT_2
            }
        })
        .hole(Circle::new(0.5))
        .split(|shape| shape.fill([1.0; 4]), |shape| shape);

    let mesh = shape.generate(&cfg, ());

    println!("{:?}", mesh);

    /*
    for mut mesh in mesh {
        mesh.polygon.make_simple();
        mesh.polygon.remove_intersection();

        mesh.holes[0].make_simple();
        mesh.holes[0].remove_intersection();

        print!("polygon(");

        for point in mesh.polygon.points {
            print!("({}, {}), ", point.x, point.y);
        }

        println!(")");

        print!("polygon(");

        for point in mesh.holes.pop().unwrap().points {
            print!("({}, {}), ", point.x, point.y);
        }

        println!(")");
    }
    */
}
