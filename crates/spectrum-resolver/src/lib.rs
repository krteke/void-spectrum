//! Theme merging, reference resolution, and contract validation.
//!
//! The resolver will consume partial schema values and produce complete
//! strongly typed values from `spectrum-core`.

mod color;
mod error;
mod font_style;
mod font_weight;
mod length;
mod line_height;
mod radius;
mod shadow;
mod theme;

pub use color::{ColorBinding, resolve_colors};
pub use error::ResolveError;
pub use font_style::resolve_font_styles;
pub use font_weight::resolve_font_weights;
pub use length::resolve_lengths;
pub use line_height::resolve_line_heights;
pub use radius::resolve_radii;
pub use shadow::resolve_shadows;
pub use theme::{ResolvedTheme, resolve_theme};
