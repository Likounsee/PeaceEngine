//! PeaceEngine is a reusable, application-agnostic graphics engine.
//!
//! The public API is independent from any host UI framework. Applications
//! own their event loops and provide presentation surfaces to the engine.

pub mod graphics;

pub use graphics::{Color, Point, Rect, Size, Surface};
