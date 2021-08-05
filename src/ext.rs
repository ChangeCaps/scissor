use glam::Vec2;

use crate::{mesh::Mesh, polygon::Polygon, polyline::Polyline, shapes::*, Shape};

/// Extension trait to [`Shape`] to make code simpler to write.
pub trait ShapeExt: Shape + Sized {
    /// Splits self into two branches, their output later to be combined.
    ///
    /// Useful for when a main color is needed as well as outlines.
    #[inline]
    fn split<
        T: Shape<Input = Self::Output>,
        F1: Fn(Id<Self::Output>) -> T,
        U: Shape<Input = Self::Output>,
        F2: Fn(Id<Self::Output>) -> U,
    >(
        self,
        f1: F1,
        f2: F2,
    ) -> Combine<Self, Split<T, U>> {
        Combine {
            input: self,
            output: Split {
                t: f1(Id::new()),
                u: f2(Id::new()),
            },
        }
    }

    #[inline]
    fn forward(self, length: f32) -> Combine<Self, Forward>
    where
        Self: Shape<Output = Polyline>,
    {
        Combine {
            input: self,
            output: Forward { length },
        }
    }

    #[inline]
    fn turn(self, radius: f32, angle: f32) -> Combine<Self, Turn>
    where
        Self: Shape<Output = Polyline>,
    {
        Combine {
            input: self,
            output: Turn { radius, angle },
        }
    }

    #[inline]
    fn offset(self, offset: f32) -> Combine<Self, Offset<Self::Output>>
    where
        Offset<Self::Output>: Shape,
    {
        Combine {
            input: self,
            output: Offset::new(offset),
        }
    }

    /// Thickens line, potentially rounded.
    #[inline]
    fn thicken(self, thickness: f32, round: bool) -> Combine<Self, Thicken>
    where
        Self: Shape<Output = Polyline>,
    {
        Combine {
            input: self,
            output: Thicken { thickness, round },
        }
    }

    /// Attaches end to start of line, thus *completing* the polygon.
    #[inline]
    fn complete(self) -> Combine<Self, Complete>
    where
        Self: Shape<Output = Polyline>,
    {
        Combine {
            input: self,
            output: Complete,
        }
    }

    /// Creates hole in polygon.
    #[inline]
    fn hole<H: Shape<Output = Polygon>>(self, hole: H) -> Combine<Self, Hole<H>>
    where
        Self: Shape<Output = Polygon>,
    {
        Combine {
            input: self,
            output: Hole { hole },
        }
    }

    /// Runs closure for each point in a polygon.
    ///
    /// This breaks any guarantees about the polygon requiring a complete re-verification, thus
    /// often leading to a greater performance loss than apparently obvious.
    #[inline]
    fn map<F: Fn(&mut Vec2)>(self, f: F) -> Combine<Self, MapPolygon<F>>
    where
        Self: Shape<Output = Polygon>,
    {
        Combine {
            input: self,
            output: MapPolygon { f },
        }
    }

    /// Combines two meshes.
    #[inline]
    fn combine(self) -> Combine<Self, CombineMeshes>
    where
        Self: Shape<Output = (Mesh, Mesh)>,
    {
        Combine {
            input: self,
            output: CombineMeshes,
        }
    }

    /// Outlines the shape.
    ///
    /// **Note** this is quite costly.
    #[inline]
    fn outline(self, thickness: f32) -> Combine<Self, Outline<Self::Output>>
    where
        Outline<Self::Output>: Shape,
    {
        Combine {
            input: self,
            output: Outline::new(thickness),
        }
    }

    /// Triangulates polygon, thus *filling* them.
    ///
    /// **Note** this is quite costly.
    #[inline]
    fn fill(self, color: impl Into<[f32; 4]>) -> Combine<Self, Fill<Self::Output>>
    where
        Fill<Self::Output>: Shape,
    {
        Combine {
            input: self,
            output: Fill::new(color.into()),
        }
    }
}

impl<T: Shape> ShapeExt for T {}
