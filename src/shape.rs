/// Configuration for generation.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// Approximate maximum distance between points.
    pub resolution: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self { resolution: 1.0 }
    }
}

pub trait Shape {
    type Input;
    type Output;

    fn generate(&self, cfg: &Config, input: Self::Input) -> Self::Output;
}
