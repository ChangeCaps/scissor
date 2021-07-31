use crate::{mesh::Mesh, Config, Shape};

#[derive(Clone, Debug)]
pub struct Combine<I, O> {
    pub input: I,
    pub output: O,
}

impl<I, O> Shape for Combine<I, O>
where
    I: Shape,
    O: Shape<Input = I::Output>,
{
    type Input = I::Input;
    type Output = O::Output;

    #[inline]
    fn generate(&self, cfg: &crate::Config, input: Self::Input) -> Self::Output {
        self.output.generate(cfg, self.input.generate(cfg, input))
    }
}

#[derive(Clone, Debug)]
pub struct CombineMesh<T> {
    pub mesh: T,
}

impl<T> Shape for CombineMesh<T>
where
    T: Shape<Input = (), Output = Mesh>,
{
    type Input = Mesh;
    type Output = Mesh;

    #[inline]
    fn generate(&self, cfg: &Config, mut mesh: Self::Input) -> Self::Output {
        let mut add_mesh = self.mesh.generate(cfg, ());

        let index = mesh.vertices.len() as u32;

        mesh.vertices.append(&mut add_mesh.vertices);
        add_mesh
            .indices
            .into_iter()
            .for_each(|i| mesh.indices.push(index + i));

        mesh
    }
}

#[derive(Clone, Debug)]
pub struct CombineMeshes;

impl Shape for CombineMeshes {
    type Input = (Mesh, Mesh);
    type Output = Mesh;

    fn generate(&self, _cfg: &Config, (mut a, mut b): Self::Input) -> Self::Output {
        let index = a.vertices.len() as u32;

        a.vertices.append(&mut b.vertices);
        b.indices
            .into_iter()
            .for_each(|i| a.indices.push(index + i));

        a
    }
}
