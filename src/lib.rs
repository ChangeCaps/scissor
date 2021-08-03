//! A crate for generating shapes through different operations.
//!
//! # Example
//! This is the code used to create the logo.
//! ```
//! // create curve that starts at the bottom, we do this to ensure a vertex is placed
//! // at the point
//! let shape = Parametric::new(|x| Vec2::new(x.sin(), -x.cos()), 0.0..TAU)
//!     // complete the circle
//!     .complete()
//!     .map(|v| {
//!         let f = PI / 4.0;
//!
//!         // if the point is under the line, move it to form the point
//!         if v.y < -f.sin() {
//!             v.y = v.x.abs() - SQRT_2;
//!         }
//!     })
//!     // create a hole in the center
//!     .hole(Circle::new(0.4))
//!     // split the shape
//!     .split(
//!         // the background is filled in
//!         |shape| shape.fill([0.1, 0.2, 0.6, 1.0]),
//!         // outline the mesh and fill it with black
//!         |shape| shape.outline(0.1).fill([0.0, 0.0, 0.0, 1.0]),
//!     )
//!     // combine the background and outline into a single mesh
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

pub use ext::ShapeExt;
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
