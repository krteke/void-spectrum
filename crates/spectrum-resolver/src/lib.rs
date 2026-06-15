//! Theme merging, reference resolution, and contract validation.
//!
//! The resolver will consume partial schema values and produce complete
//! strongly typed values from `spectrum-core`.

mod color;
mod error;
mod length;
mod radius;
mod theme;

pub use color::{ColorBinding, resolve_colors};
pub use error::ResolveError;
pub use length::resolve_lengths;
pub use radius::resolve_radii;
pub use theme::{ResolvedTheme, resolve_theme};
