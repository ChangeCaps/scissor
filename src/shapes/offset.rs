use glam::{Vec2};

use crate::{Polyline, Shape};

#[derive(Clone, Debug)]
pub struct Offset<T> {
	pub offset: f32,
	_marker: std::marker::PhantomData<*const T>,
}

impl<T> Offset<T> {
	#[inline]
	pub const fn new(offset: f32) -> Self {
		Self {
			offset,
			_marker: std::marker::PhantomData,
		}
	}
}

impl Shape for Offset<Polyline> {
    type Input = Polyline;
    type Output = Polyline;

	#[inline]
    fn generate(&self, _cfg: &crate::Config, input: Self::Input) -> Self::Output {	
		#[inline]
        fn nor(v: Vec2) -> Vec2 {
            Vec2::new(-v.y, v.x)
        }

		let mut line = Polyline::default();

		{
			let p0 = input.points[0];
			let p1 = input.points[1];

			let n = nor(p1 - p0).normalize();

			line.push(p0 - n * self.offset);
		}

		for i in 1..input.points.len() - 1 {
			let p0 = input.points[i - 1];
			let p1 = input.points[i];
			let p2 = input.points[i + 1];

			let n0 = nor(p1 - p0).normalize();
			let n1 = nor(p2 - p1).normalize();

			let n = ((n0 + n1) / 2.0).normalize();

			line.push(p1 - n * self.offset);
		}

		{
			let p0 = input.points[input.points.len() - 1];
			let p1 = input.points[input.points.len() - 2];

			let n = nor(p1 - p0).normalize();

			line.push(p0 + n * self.offset);
		}

		line
    }
}