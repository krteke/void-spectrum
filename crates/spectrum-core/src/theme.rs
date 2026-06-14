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
    /// Container using the primary color domain.
    pub primary_container: Color,
    /// Foreground content rendered on the primary container.
    pub on_primary_container: Color,
    /// Secondary accent used for less prominent emphasis.
    pub secondary: Color,
    /// Foreground content rendered on the secondary color.
    pub on_secondary: Color,
    /// Container using the secondary color domain.
    pub secondary_container: Color,
    /// Foreground content rendered on the secondary container.
    pub on_secondary_container: Color,
    /// Tertiary accent used for contrasting emphasis.
    pub tertiary: Color,
    /// Foreground content rendered on the tertiary color.
    pub on_tertiary: Color,
    /// Container using the tertiary color domain.
    pub tertiary_container: Color,
    /// Foreground content rendered on the tertiary container.
    pub on_tertiary_container: Color,
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
    /// Container used for error and destructive states.
    pub error_container: Color,
    /// Foreground content rendered on the error container.
    pub on_error_container: Color,
}
