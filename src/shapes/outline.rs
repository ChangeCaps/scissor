use glam::Vec2;

use crate::{holed_polygon::HoledPolygon, polygon::Polygon, Config, Shape};

#[inline]
fn nor(v: Vec2) -> Vec2 {
    Vec2::new(-v.y, v.x)
}

#[inline]
fn offset(polygon: &Polygon, thickness: f32) -> Polygon {
    let mut offset = Polygon::with_capacity(polygon.points.len());

    let len = polygon.points.len();
    for i in 1..=len {
        let p0 = polygon.points[i - 1];
        let p1 = polygon.points[i % len];
        let p2 = polygon.points[(i + 1) % len];

        let n0 = nor((p1 - p0).normalize());
        let n1 = nor((p2 - p1).normalize());

        let n = ((n0 + n1) / 2.0).normalize();

        let t = thickness / n0.dot(n);

        let p = p1 + n * t;

        offset.push(p);
    }

    offset
}

/// Outlines a [`Shape`].
#[derive(Clone, Debug)]
pub struct Outline<T> {
    pub thickness: f32,
    _marker: std::marker::PhantomData<*const T>,
}

impl<T> Outline<T> {
    #[inline]
    pub const fn new(thickness: f32) -> Self {
        Self {
            thickness,
            _marker: std::marker::PhantomData,
        }
    }
}

impl Shape for Outline<Polygon> {
    type Input = Polygon;
    type Output = HoledPolygon;

    #[inline]
    fn generate(&self, _cfg: &Config, mut input: Self::Input) -> Self::Output {
        input.verify();

        let hole = offset(&input, self.thickness / 2.0);
        let mut polygon = offset(&input, -self.thickness / 2.0);

        if input.is_convex() {
            polygon.is_simple = true;
            polygon.is_ccw = Some(true);
            polygon.is_convex = Some(true);
        }

        HoledPolygon {
            polygon,
            holes: vec![hole],
        }
    }
}

impl Shape for Outline<HoledPolygon> {
    type Input = HoledPolygon;
    type Output = Vec<HoledPolygon>;

    #[inline]
    fn generate(&self, _cfg: &Config, mut input: Self::Input) -> Self::Output {
        input.verify();

        let mut polygons = Vec::with_capacity(input.holes.len() + 1);

        for hole in input.holes {
            let hole = offset(&hole, self.thickness);
            let mut polygon = offset(&hole, -self.thickness);

            if input.polygon.is_convex() {
                polygon.is_simple = true;
                polygon.is_convex = Some(true);
                polygon.is_ccw = Some(true);
            }

            polygons.push(HoledPolygon {
                polygon,
                holes: vec![hole],
            });
        }

        let hole = offset(&input.polygon, self.thickness / 2.0);
        let mut polygon = offset(&input.polygon, -self.thickness / 2.0);

        if input.polygon.is_convex() {
            polygon.is_simple = true;
            polygon.is_convex = Some(true);
            polygon.is_ccw = Some(true);
        }

        polygons.push(HoledPolygon {
            polygon,
            holes: vec![hole],
        });

        polygons
    }
}
