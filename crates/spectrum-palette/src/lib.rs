//! Seed-color and tonal-palette generation boundary.

#[cfg(feature = "seed")]
mod tonal;
#[cfg(feature = "color-spaces")]
mod transform;

#[cfg(feature = "seed")]
pub use tonal::TonalPalette;
#[cfg(feature = "color-spaces")]
pub use transform::ColorExt;
