use crate::{polygon::Polygon, Shape};
use std::any::Any;

#[derive(Clone, Debug)]
pub struct Split<T, U> {
    pub t: T,
    pub u: U,
}

impl<T, U> Shape for Split<T, U>
where
    T: Shape,
    U: Shape<Input = T::Input>,
    T::Input: Clone + Any,
{
    type Input = T::Input;
    type Output = (T::Output, U::Output);

    #[inline]
    fn generate(&self, cfg: &crate::Config, mut input: Self::Input) -> Self::Output {
        if let Some(input) = <dyn Any>::downcast_mut::<Polygon>(&mut input) {
            input.verify();
        }

        (
            self.t.generate(cfg, input.clone()),
            self.u.generate(cfg, input),
        )
    }
}
