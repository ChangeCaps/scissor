use std::f32::consts::PI;

use glam::Vec2;

use crate::{polygon::Polygon, polyline::Polyline, Config, Shape};

#[derive(Clone, Debug)]
pub struct Thicken {
    pub thickness: f32,
    pub round: bool,
}

impl Shape for Thicken {
    type Input = Polyline;
    type Output = Polygon;

    #[inline]
    fn generate(&self, cfg: &Config, line: Self::Input) -> Self::Output {
        #[inline]
        fn push_cap(polygon: &mut Polygon, cfg: &Config, p: Vec2, thickness: f32, angle: f32) {
            let dist = PI * thickness;
            let steps = (dist / cfg.resolution).ceil() as usize;

            for step in 0..=steps {
                let l = step as f32 / steps as f32;
                let a = angle + l * PI;

                let p = p + Vec2::new(a.cos(), a.sin()) * thickness;

                polygon.push(p);
            }
        }

        #[inline]
        fn nor(v: Vec2) -> Vec2 {
            Vec2::new(-v.y, v.x)
        }

        let mut polygon = Polygon::default();

        // starting cap
        let p0 = line.points[0];
        let p1 = line.points[1];

        let d = p1 - p0;
        let a = d.y.atan2(d.x) + PI / 2.0;

        if self.round {
            push_cap(&mut polygon, cfg, p0, self.thickness / 2.0, a);
        }

        for i in 1..line.points.len() - 1 {
            let p0 = line.points[i - 1];
            let p1 = line.points[i];
            let p2 = line.points[i + 1];

            let n0 = nor((p1 - p0).normalize());
            let n1 = nor((p2 - p1).normalize());

            let n = (n0 + n1).normalize();

            let p = p1 - n * self.thickness / 2.0;

            polygon.push(p);
        }

        // ending cap
        let p0 = line.points[line.points.len() - 1];
        let p1 = line.points[line.points.len() - 2];

        let d = p1 - p0;
        let a = d.y.atan2(d.x) + PI / 2.0;

        if self.round {
            push_cap(&mut polygon, cfg, p0, self.thickness / 2.0, a);
        }

        for i in (1..line.points.len() - 1).rev() {
            let p0 = line.points[i - 1];
            let p1 = line.points[i];
            let p2 = line.points[i + 1];

            let n0 = nor((p1 - p0).normalize());
            let n1 = nor((p2 - p1).normalize());

            let n = (n0 + n1).normalize();

            let p = p1 + n * self.thickness / 2.0;

            polygon.push(p);
        }

        polygon.remove_intersection();
        polygon.make_simple();

        polygon
    }
}
