use glam::Vec2;

use crate::{polygon::Polygon, Shape};

#[derive(Clone, Debug)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl Shape for Rect {
    type Input = ();
    type Output = Polygon;

    fn generate(&self, _cfg: &crate::Config, _input: Self::Input) -> Self::Output {
        let mut polygon = Polygon::default();

        polygon.push(Vec2::new(-self.width / 2.0, -self.height / 2.0));
        polygon.push(Vec2::new(self.width / 2.0, -self.height / 2.0));
        polygon.push(Vec2::new(self.width / 2.0, self.height / 2.0));
        polygon.push(Vec2::new(-self.width / 2.0, self.height / 2.0));

        polygon.is_ccw = Some(true);
        polygon.is_convex = Some(true);
        polygon.is_simple = true;

        polygon
    }
}
