use glam::Vec3;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: [f32; 4],
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Vertex {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Vertex {}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}
