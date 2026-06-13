//! Platform-independent value types and resolved theme contracts.
//!
//! ```
//! use spectrum_core::Color;
//!
//! let accent: Color = "#7c3aed".parse()?;
//! assert_eq!(accent.to_string(), "#7c3aed");
//! # Ok::<(), spectrum_core::ColorParseError>(())
//! ```

mod color;
mod font;
mod length;
mod radius;

pub use color::{Color, ColorParseError};
pub use font::{FontWeight, FontWeightParseError};
pub use length::{Length, LengthParseError, LengthUnit};
pub use radius::{Radius, RadiusParseError};
