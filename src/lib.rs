//! A crate for generating shapes through different operations.
//!
//! # Example
//! ```
//! let shape = Parametric::new(|x| Vec2::new(x.sin(), -x.cos()), 0.0..TAU)
//!     .complete()
//!     .map(|v| {
//!         let f = PI / 4.0;
//!
//!         let p = f.sin();
//!
//!         if v.y < -p {
//!             v.y = v.x.abs() / f.tan() - SQRT_2;
//!         }
//!     })
//!     .hole(Circle::new(0.4))
//!     .split(
//!         |shape| shape.fill([0.1, 0.2, 0.6, 1.0]),
//!         |shape| shape.outline(0.1).fill([0.0, 0.0, 0.0, 1.0]),
//!     )
//!     .combine();
//! ```
//!
#![doc(html_logo_url = "https://raw.githubusercontent.com/ChangeCaps/scissor/main/logo.png")]

mod ext;
mod holed_polygon;
pub mod mesh;
mod polygon;
mod polyline;
mod shape;
pub mod shapes;

#[doc(hidden)]
pub use ext::*;
#[doc(hidden)]
pub use glam;
pub use holed_polygon::HoledPolygon;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use shape::{Config, Shape};
#[doc(hidden)]
pub use shapes::*;

pub mod prelude {
    pub use crate::ext::*;
    pub use crate::shape::Shape;
    pub use crate::shapes::*;

    pub use glam::{swizzles::*, *};
}
