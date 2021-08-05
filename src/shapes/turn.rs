use glam::Mat2;

use crate::{Polyline, Shape};

#[derive(Clone, Debug)]
pub struct Turn {
	pub radius: f32,
	pub angle: f32,
}

impl Shape for Turn {
	type Input = Polyline;
	type Output = Polyline;

	#[inline]
	fn generate(&self, cfg: &crate::Config, mut input: Self::Input) -> Self::Output {
		let dist = self.angle.abs() * self.radius;
		let steps = (dist / cfg.resolution).ceil() as usize;

		let angle_per_step = self.angle / steps as f32;
		let dist_per_step = (angle_per_step.abs() / 2.0).sin() * self.radius * 2.0;

		let rot = Mat2::from_angle(angle_per_step);

		let mut p = input.points[input.points.len() - 1];
		let mut vec = (p - input.points[input.points.len() - 2]).normalize() * dist_per_step;

		for _ in 0..steps {
			vec = rot * vec;
			p += vec;

			input.push(p);
		}

		input.direction = Some(vec.normalize());

		input
	}	
}