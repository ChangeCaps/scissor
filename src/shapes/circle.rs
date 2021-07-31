use std::f32::consts::TAU;

use glam::Vec2;

use crate::{polygon::Polygon, Shape};

#[derive(Clone, Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    #[inline]
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    type Input = ();
    type Output = Polygon;

    #[inline]
    fn generate(&self, cfg: &crate::Config, _: Self::Input) -> Self::Output {
        let dist = self.radius * TAU;
        let steps = (dist / cfg.resolution).ceil() as usize;

        let mut polygon = Polygon::default();

        for step in 0..steps {
            let a = (step as f32 / steps as f32) * TAU;

            polygon.push(Vec2::new(a.cos(), a.sin()) * self.radius);
        }

        polygon.is_ccw = Some(true);
        polygon.is_convex = Some(true);
        polygon.is_simple = true;

        polygon
    }
}
