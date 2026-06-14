use spectrum_core::Color;

/// A stable role in the Material color source namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialColor {
    /// Primary accent.
    Primary,
    /// Content on the primary accent.
    OnPrimary,
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
}

impl MaterialColor {
    /// Parses the segment after the `material.` namespace prefix.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "primary" => Some(Self::Primary),
            "on_primary" => Some(Self::OnPrimary),
            "background" => Some(Self::Background),
            "on_background" => Some(Self::OnBackground),
            "surface" => Some(Self::Surface),
            "on_surface" => Some(Self::OnSurface),
            "error" => Some(Self::Error),
            "on_error" => Some(Self::OnError),
            _ => None,
        }
    }
}

/// Material role values generated for one Seed and theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialColors(pub(crate) [Color; 8]);

impl MaterialColors {
    /// Returns the generated value for a Material role.
    #[must_use]
    pub const fn resolve(self, role: MaterialColor) -> Color {
        self.0[role as usize]
    }
}
