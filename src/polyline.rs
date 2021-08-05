use glam::Vec2;

/// Multiple lines connected.
///
/// **Must** contain two or more points.
#[derive(Clone, Debug, Default)]
pub struct Polyline {
    pub points: Vec<Vec2>,
    pub direction: Option<Vec2>,
}

impl Polyline {
    #[inline]
    pub fn push(&mut self, point: Vec2) {
        self.points.push(point);
        self.direction = None;
    }

    #[inline]
    pub fn direction(&self) -> Vec2 {
        if let Some(direction) = self.direction {
            direction
        } else {
            let p1 = self.points[self.points.len() - 1];
            let p0 = self.points[self.points.len() - 2];

            (p1 - p0).normalize()
        }
    }
}

impl From<Vec<Vec2>> for Polyline {
    fn from(points: Vec<Vec2>) -> Self {
        Self { points, direction: None }
    }
}
