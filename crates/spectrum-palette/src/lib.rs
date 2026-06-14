//! Seed-color and tonal-palette generation boundary.

#[cfg(feature = "seed")]
mod domains;
mod material;
#[cfg(feature = "seed")]
mod semantic;
#[cfg(feature = "seed")]
mod tonal;
#[cfg(feature = "color-spaces")]
mod transform;

pub use material::{MaterialColor, MaterialColors};
#[cfg(feature = "seed")]
pub use semantic::{material_colors, semantic_colors};
#[cfg(feature = "seed")]
pub use tonal::TonalPalette;
#[cfg(feature = "color-spaces")]
pub use transform::ColorExt;
