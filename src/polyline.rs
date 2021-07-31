use glam::Vec2;

/// Multiple lines connected.
///
/// **Must** contain two or more points.
#[derive(Clone, Debug, Default)]
pub struct Polyline {
    pub points: Vec<Vec2>,
}

impl Polyline {
    #[inline]
    pub fn push(&mut self, point: Vec2) {
        self.points.push(point);
    }
}

impl From<Vec<Vec2>> for Polyline {
    fn from(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}
