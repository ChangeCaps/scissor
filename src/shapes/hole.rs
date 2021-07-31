use crate::{holed_polygon::HoledPolygon, polygon::Polygon, Config, Shape};

#[derive(Clone, Debug)]
pub struct Hole<H: Shape<Output = Polygon>> {
    pub hole: H,
}

impl<H> Shape for Hole<H>
where
    H: Shape<Input = (), Output = Polygon>,
{
    type Input = Polygon;
    type Output = HoledPolygon;

    #[inline]
    fn generate(&self, cfg: &Config, polygon: Self::Input) -> Self::Output {
        HoledPolygon {
            polygon,
            holes: vec![self.hole.generate(cfg, ())],
        }
    }
}
