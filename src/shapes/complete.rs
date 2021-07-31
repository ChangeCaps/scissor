use crate::{polygon::Polygon, polyline::Polyline, Config, Shape};

/// Converts a [`Polyline`] to a [`Polygon`], by connecting the start and end points.
#[derive(Clone, Debug)]
pub struct Complete;

impl Shape for Complete {
    type Input = Polyline;
    type Output = Polygon;

    #[inline]
    fn generate(&self, _cfg: &Config, polyline: Self::Input) -> Self::Output {
        Polygon::from(polyline.points)
    }
}
