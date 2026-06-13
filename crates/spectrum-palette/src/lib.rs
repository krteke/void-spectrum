//! Seed-color and tonal-palette generation boundary.
//!
//! The concrete Material 3 compatible color algorithm remains intentionally
//! unselected until reference-value tests are available.

#[cfg(feature = "color-spaces")]
mod transform;

#[cfg(feature = "color-spaces")]
pub use transform::ColorExt;
