//! Serializable, partially specified theme configuration contracts.
//!
//! This crate will model external theme files without coupling configuration
//! data to resolver behavior or rendering frameworks.

mod meta;

pub use meta::{ThemeMeta, ThemeMode};
