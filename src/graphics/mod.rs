//! Platform-independent graphics primitives and rendering backends.

mod color;
mod geometry;
mod surface;

#[cfg(feature = "software-renderer")]
mod software;

pub use color::Color;
pub use geometry::{Point, Rect, Size};
pub use surface::{Surface, SurfaceError};

#[cfg(feature = "software-renderer")]
pub use software::SoftwareRenderer;
