use glam::Vec2;

use crate::{
    holed_polygon::HoledPolygon, mesh::Mesh, polygon::Polygon, polyline::Polyline, shapes::*, Shape,
};

pub trait PolylineExt: Shape<Output = Polyline> + Sized {
    #[inline]
    fn thicken(self, thickness: f32) -> Combine<Self, Thicken> {
        Combine {
            input: self,
            output: Thicken { thickness },
        }
    }

    #[inline]
    fn complete(self) -> Combine<Self, Complete> {
        Combine {
            input: self,
            output: Complete,
        }
    }
}

pub trait PolygonExt: Shape<Output = Polygon> + Sized {
    #[inline]
    fn outline(self, thickness: f32) -> Combine<Self, Outline> {
        Combine {
            input: self,
            output: Outline { thickness },
        }
    }

    #[inline]
    fn fill(self, color: impl Into<[f32; 4]>) -> Combine<Self, FillPolygon> {
        Combine {
            input: self,
            output: FillPolygon {
                color: color.into(),
            },
        }
    }

    #[inline]
    fn hole<H: Shape<Output = Polygon>>(self, hole: H) -> Combine<Self, Hole<H>> {
        Combine {
            input: self,
            output: Hole { hole },
        }
    }

    #[inline]
    fn map<F: Fn(&mut Vec2)>(self, f: F) -> Combine<Self, MapPolygon<F>> {
        Combine {
            input: self,
            output: MapPolygon { f },
        }
    }
}

pub trait HoledPolygonExt: Shape<Output = HoledPolygon> + Sized {
    #[inline]
    fn outline(self, thickness: f32) -> Combine<Self, OutlineHoled> {
        Combine {
            input: self,
            output: OutlineHoled { thickness },
        }
    }

    #[inline]
    fn fill(self, color: impl Into<[f32; 4]>) -> Combine<Self, FillHoled> {
        Combine {
            input: self,
            output: FillHoled {
                color: color.into(),
            },
        }
    }
}

pub trait PolygonsExt: Shape<Output = Vec<HoledPolygon>> + Sized {
    #[inline]
    fn fill(self, color: impl Into<[f32; 4]>) -> Combine<Self, FillPolygons> {
        Combine {
            input: self,
            output: FillPolygons {
                color: color.into(),
            },
        }
    }
}

pub trait MeshExt: Shape<Output = Mesh> + Sized {
    #[inline]
    fn combine<T: Shape<Input = (), Output = Mesh>>(
        self,
        mesh: T,
    ) -> Combine<Self, CombineMesh<T>> {
        Combine {
            input: self,
            output: CombineMesh { mesh },
        }
    }
}

pub trait ShapeExt: Shape + Sized {
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
    fn combine(self) -> Combine<Self, CombineMeshes>
    where
        Self: Shape<Output = (Mesh, Mesh)>,
    {
        Combine {
            input: self,
            output: CombineMeshes,
        }
    }
}

impl<T: Shape<Output = Polyline> + Sized> PolylineExt for T {}
impl<T: Shape<Output = Polygon> + Sized> PolygonExt for T {}
impl<T: Shape<Output = HoledPolygon> + Sized> HoledPolygonExt for T {}
impl<T: Shape<Output = Vec<HoledPolygon>> + Sized> PolygonsExt for T {}
impl<T: Shape<Output = Mesh> + Sized> MeshExt for T {}
impl<T: Shape> ShapeExt for T {}
