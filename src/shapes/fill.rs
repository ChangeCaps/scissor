use crate::{
    holed_polygon::HoledPolygon,
    mesh::{Mesh, Vertex},
    polygon::Polygon,
    Config, Shape,
};

#[derive(Clone, Debug)]
pub struct FillPolygon {
    pub color: [f32; 4],
}

impl Shape for FillPolygon {
    type Input = Polygon;
    type Output = Mesh;

    #[inline]
    fn generate(&self, _cfg: &Config, mut polygon: Self::Input) -> Self::Output {
        polygon.verify();

        let indices = polygon
            .triangulate()
            .into_iter()
            .map(|i| i as u32)
            .collect();

        let vertices = polygon
            .points
            .into_iter()
            .map(|p| Vertex {
                position: p.extend(0.0),
                color: self.color,
            })
            .collect();

        Mesh { vertices, indices }
    }
}

#[derive(Clone, Debug)]
pub struct FillHoled {
    pub color: [f32; 4],
}

impl Shape for FillHoled {
    type Input = HoledPolygon;
    type Output = Mesh;

    #[inline]
    fn generate(&self, _cfg: &Config, mut polygon: Self::Input) -> Self::Output {
        polygon.verify();

        for hole in polygon.holes {
            polygon.polygon.merge_hole(hole);
        }

        let indices = polygon
            .polygon
            .triangulate()
            .into_iter()
            .map(|i| i as u32)
            .collect();

        let vertices = polygon
            .polygon
            .points
            .into_iter()
            .map(|p| Vertex {
                position: p.extend(0.0),
                color: self.color,
            })
            .collect();

        Mesh { vertices, indices }
    }
}

#[derive(Clone, Debug)]
pub struct FillPolygons {
    pub color: [f32; 4],
}

impl Shape for FillPolygons {
    type Input = Vec<HoledPolygon>;
    type Output = Mesh;

    #[inline]
    fn generate(&self, _cfg: &Config, polygons: Self::Input) -> Self::Output {
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        let mut index = 0;

        for mut polygon in polygons {
            polygon.verify();

            for hole in polygon.holes {
                polygon.polygon.merge_hole(hole);
            }

            polygon
                .polygon
                .triangulate()
                .into_iter()
                .for_each(|i| indices.push(i as u32 + index));

            polygon.polygon.points.into_iter().for_each(|p| {
                vertices.push(Vertex {
                    position: p.extend(0.0),
                    color: self.color,
                })
            });

            index = vertices.len() as u32;
        }

        Mesh { vertices, indices }
    }
}
