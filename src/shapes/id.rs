use crate::Shape;

#[derive(Clone, Debug, Default)]
pub struct Id<T>(std::marker::PhantomData<*const T>);

impl<T> Id<T> {
    pub const ID: Self = Self(std::marker::PhantomData);

    #[inline]
    pub const fn new() -> Self {
        Self::ID
    }
}

impl<T> Shape for Id<T> {
    type Input = T;
    type Output = T;

    #[inline]
    fn generate(&self, _cfg: &crate::Config, input: Self::Input) -> Self::Output {
        input
    } 
}
