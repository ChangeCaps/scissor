use glam::Vec2;

use crate::{polygon::Polygon, Shape};

#[derive(Clone, Debug)]
pub struct MapPolygon<F: Fn(&mut Vec2)> {
    pub f: F,
}

impl<F: Fn(&mut Vec2)> Shape for MapPolygon<F> {
    type Input = Polygon;
    type Output = Polygon;

    #[inline]
    fn generate(&self, _cfg: &crate::Config, mut polygon: Self::Input) -> Self::Output {
        polygon.points.iter_mut().for_each(&self.f);

        polygon.is_simple = false;
        polygon.is_ccw = None;
        polygon.is_convex = None;

        polygon
    }
}
