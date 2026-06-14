//! Serializable, partially specified theme configuration contracts.
//!
//! This crate will model external theme files without coupling configuration
//! data to resolver behavior or rendering frameworks.

mod meta;
mod spec;
mod value;

pub use meta::ThemeMeta;
pub use spec::ThemeSpec;
pub use spectrum_core::ThemeMode;
pub use value::{ColorValue, ColorValueParseError, TokenReference};
