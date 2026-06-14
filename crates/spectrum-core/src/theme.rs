use core::fmt;

use crate::Color;

/// Whether a theme targets a light or dark surface.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    /// A theme intended for dark surfaces.
    #[default]
    Dark,
    /// A theme intended for light surfaces.
    Light,
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Dark => "dark",
            Self::Light => "light",
        })
    }
}

/// Core semantic colors derived from a theme seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticColors {
    /// Primary accent used for prominent controls and emphasis.
    pub primary: Color,
    /// Foreground content rendered on the primary color.
    pub on_primary: Color,
    /// Lowest-level application background.
    pub background: Color,
    /// Foreground content rendered on the application background.
    pub on_background: Color,
    /// Default color for component surfaces.
    pub surface: Color,
    /// Foreground content rendered on component surfaces.
    pub on_surface: Color,
    /// Color used for errors and destructive states.
    pub error: Color,
    /// Foreground content rendered on the error color.
    pub on_error: Color,
}
