use crate::polygon::Polygon;

/// A polygon with holes.
#[derive(Clone, Debug)]
pub struct HoledPolygon {
    pub polygon: Polygon,
    pub holes: Vec<Polygon>,
}

impl HoledPolygon {
    #[inline]
    pub fn verify(&mut self) {
        self.polygon.verify();

        for hole in &mut self.holes {
            hole.verify();
        }
    }
}

impl From<Polygon> for HoledPolygon {
    fn from(polygon: Polygon) -> Self {
        Self {
            polygon,
            holes: Vec::new(),
        }
    }
}
