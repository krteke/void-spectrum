//! Theme merging, reference resolution, and contract validation.
//!
//! The resolver will consume partial schema values and produce complete
//! strongly typed values from `spectrum-core`.

mod color;
mod error;

pub use color::resolve_colors;
pub use error::ResolveError;
