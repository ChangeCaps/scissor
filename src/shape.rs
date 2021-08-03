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

/// Central trait that takes in an input and generates and output from it.
pub trait Shape {
    type Input;
    type Output;

    fn generate(&self, cfg: &Config, input: Self::Input) -> Self::Output;
}
