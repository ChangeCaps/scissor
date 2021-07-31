use std::ops::Range;

use glam::Vec2;

use crate::{polyline::Polyline, Config, Shape};

#[derive(Clone, Debug)]
pub struct Parametric<F: Fn(f32) -> Vec2> {
    pub f: F,
    pub interval: Range<f32>,
}

impl<F: Fn(f32) -> Vec2> Parametric<F> {
    #[inline]
    pub fn new(f: F, interval: Range<f32>) -> Self {
        Self { f, interval }
    }
}

impl<F: Fn(f32) -> Vec2> Shape for Parametric<F> {
    type Input = ();
    type Output = Polyline;

    #[inline]
    fn generate(&self, cfg: &Config, _: Self::Input) -> Self::Output {
        let dist = self.interval.end - self.interval.start;
        let steps = (dist / cfg.resolution).ceil() as usize;

        let mut polyline = Polyline::default();

        for step in 0..steps {
            let x = step as f32 / (steps) as f32 * dist + self.interval.start;

            let p = (self.f)(x);

            polyline.push(p);
        }

        polyline
    }
}
