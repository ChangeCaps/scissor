//! A collection of standard [Shapes][`crate::Shape`].

mod circle;
mod combine;
mod complete;
mod fill;
mod hole;
mod id;
mod line;
mod map;
mod outline;
mod parametric;
mod rect;
mod split;
mod thicken;

pub use circle::Circle;
pub use combine::{Combine, CombineMesh, CombineMeshes};
pub use complete::Complete;
pub use fill::Fill;
pub use hole::Hole;
pub use id::Id;
pub use line::Line;
pub use map::MapPolygon;
pub use outline::Outline;
pub use parametric::Parametric;
pub use rect::Rect;
pub use split::Split;
pub use thicken::Thicken;
