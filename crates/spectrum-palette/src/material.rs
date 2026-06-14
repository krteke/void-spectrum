use spectrum_core::Color;

/// A stable role in the Material color source namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialColor {
    /// Primary accent.
    Primary,
    /// Content on the primary accent.
    OnPrimary,
    /// Container using the primary color domain.
    PrimaryContainer,
    /// Content on the primary container.
    OnPrimaryContainer,
    /// Secondary accent.
    Secondary,
    /// Content on the secondary accent.
    OnSecondary,
    /// Container using the secondary color domain.
    SecondaryContainer,
    /// Content on the secondary container.
    OnSecondaryContainer,
    /// Tertiary accent.
    Tertiary,
    /// Content on the tertiary accent.
    OnTertiary,
    /// Container using the tertiary color domain.
    TertiaryContainer,
    /// Content on the tertiary container.
    OnTertiaryContainer,
    /// Lowest-level application background.
    Background,
    /// Content on the application background.
    OnBackground,
    /// Default component surface.
    Surface,
    /// Content on component surfaces.
    OnSurface,
    /// Error and destructive state color.
    Error,
    /// Content on the error color.
    OnError,
    /// Container using the error color domain.
    ErrorContainer,
    /// Content on the error container.
    OnErrorContainer,
}

impl MaterialColor {
    /// Parses the segment after the `material.` namespace prefix.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "primary" => Some(Self::Primary),
            "on_primary" => Some(Self::OnPrimary),
            "primary_container" => Some(Self::PrimaryContainer),
            "on_primary_container" => Some(Self::OnPrimaryContainer),
            "secondary" => Some(Self::Secondary),
            "on_secondary" => Some(Self::OnSecondary),
            "secondary_container" => Some(Self::SecondaryContainer),
            "on_secondary_container" => Some(Self::OnSecondaryContainer),
            "tertiary" => Some(Self::Tertiary),
            "on_tertiary" => Some(Self::OnTertiary),
            "tertiary_container" => Some(Self::TertiaryContainer),
            "on_tertiary_container" => Some(Self::OnTertiaryContainer),
            "background" => Some(Self::Background),
            "on_background" => Some(Self::OnBackground),
            "surface" => Some(Self::Surface),
            "on_surface" => Some(Self::OnSurface),
            "error" => Some(Self::Error),
            "on_error" => Some(Self::OnError),
            "error_container" => Some(Self::ErrorContainer),
            "on_error_container" => Some(Self::OnErrorContainer),
            _ => None,
        }
    }
}

/// Material role values generated for one Seed and theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialColors(pub(crate) [Color; 20]);

impl MaterialColors {
    /// Returns the generated value for a Material role.
    #[must_use]
    pub const fn resolve(self, role: MaterialColor) -> Color {
        self.0[role as usize]
    }
}
