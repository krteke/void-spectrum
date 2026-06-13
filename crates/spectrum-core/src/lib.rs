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

pub use color::{Color, ColorParseError};
