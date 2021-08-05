use crate::{Polyline, Shape};


#[derive(Clone, Debug)]
pub struct Forward {
	pub length: f32,
}

impl Shape for Forward {
    type Input = Polyline;
    type Output = Polyline;

	#[inline]
    fn generate(&self, _cfg: &crate::Config, mut input: Self::Input) -> Self::Output {
		input.push(input.points[input.points.len() - 1] + input.direction() * self.length);	

		input
    }
}