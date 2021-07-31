use glam::Vec2;

use crate::{
    polyline::Polyline,
    shape::{Config, Shape},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub p0: Vec2,
    pub p1: Vec2,
}

impl Line {
    #[inline]
    pub fn new(p0: Vec2, p1: Vec2) -> Self {
        Self { p0, p1 }
    }
}

impl Shape for Line {
    type Input = ();
    type Output = Polyline;

    #[inline]
    fn generate(&self, _cfg: &Config, _: Self::Input) -> Self::Output {
        Polyline::from(vec![self.p0, self.p1])
    }
}
