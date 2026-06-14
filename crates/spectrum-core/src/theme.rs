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
    /// Dim surface used for low-emphasis areas.
    pub surface_dim: Color,
    /// Bright surface used for high-emphasis areas.
    pub surface_bright: Color,
    /// Lowest surface container level.
    pub surface_container_lowest: Color,
    /// Low surface container level.
    pub surface_container_low: Color,
    /// Default surface container level.
    pub surface_container: Color,
    /// High surface container level.
    pub surface_container_high: Color,
    /// Highest surface container level.
    pub surface_container_highest: Color,
    /// Variant surface color.
    pub surface_variant: Color,
    /// Foreground content rendered on variant surfaces.
    pub on_surface_variant: Color,
    /// Surface used in inverse color contexts.
    pub inverse_surface: Color,
    /// Foreground content rendered on the inverse surface.
    pub inverse_on_surface: Color,
    /// Default outline color.
    pub outline: Color,
    /// Lower-emphasis outline color.
    pub outline_variant: Color,
    /// Shadow color.
    pub shadow: Color,
    /// Scrim color.
    pub scrim: Color,
    /// Surface tint derived from the primary domain.
    pub surface_tint: Color,
    /// Primary accent used on inverse surfaces.
    pub inverse_primary: Color,
    /// Mode-independent primary fixed color.
    pub primary_fixed: Color,
    /// Dimmed primary fixed color.
    pub primary_fixed_dim: Color,
    /// Foreground content on primary fixed colors.
    pub on_primary_fixed: Color,
    /// Variant foreground content on primary fixed colors.
    pub on_primary_fixed_variant: Color,
    /// Mode-independent secondary fixed color.
    pub secondary_fixed: Color,
    /// Dimmed secondary fixed color.
    pub secondary_fixed_dim: Color,
    /// Foreground content on secondary fixed colors.
    pub on_secondary_fixed: Color,
    /// Variant foreground content on secondary fixed colors.
    pub on_secondary_fixed_variant: Color,
    /// Mode-independent tertiary fixed color.
    pub tertiary_fixed: Color,
    /// Dimmed tertiary fixed color.
    pub tertiary_fixed_dim: Color,
    /// Foreground content on tertiary fixed colors.
    pub on_tertiary_fixed: Color,
    /// Variant foreground content on tertiary fixed colors.
    pub on_tertiary_fixed_variant: Color,
    /// Color used for errors and destructive states.
    pub error: Color,
    /// Foreground content rendered on the error color.
    pub on_error: Color,
    /// Container used for error and destructive states.
    pub error_container: Color,
    /// Foreground content rendered on the error container.
    pub on_error_container: Color,
}
