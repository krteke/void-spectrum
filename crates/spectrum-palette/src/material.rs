use spectrum_core::{Color, ThemeMode};

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
    /// Dim surface.
    SurfaceDim,
    /// Bright surface.
    SurfaceBright,
    /// Lowest surface container.
    SurfaceContainerLowest,
    /// Low surface container.
    SurfaceContainerLow,
    /// Default surface container.
    SurfaceContainer,
    /// High surface container.
    SurfaceContainerHigh,
    /// Highest surface container.
    SurfaceContainerHighest,
    /// Variant surface.
    SurfaceVariant,
    /// Content on variant surfaces.
    OnSurfaceVariant,
    /// Inverse surface.
    InverseSurface,
    /// Content on the inverse surface.
    InverseOnSurface,
    /// Default outline.
    Outline,
    /// Lower-emphasis outline.
    OutlineVariant,
    /// Shadow color.
    Shadow,
    /// Scrim color.
    Scrim,
    /// Primary-derived surface tint.
    SurfaceTint,
    /// Primary accent on inverse surfaces.
    InversePrimary,
    /// Mode-independent primary fixed color.
    PrimaryFixed,
    /// Dimmed primary fixed color.
    PrimaryFixedDim,
    /// Content on primary fixed colors.
    OnPrimaryFixed,
    /// Variant content on primary fixed colors.
    OnPrimaryFixedVariant,
    /// Mode-independent secondary fixed color.
    SecondaryFixed,
    /// Dimmed secondary fixed color.
    SecondaryFixedDim,
    /// Content on secondary fixed colors.
    OnSecondaryFixed,
    /// Variant content on secondary fixed colors.
    OnSecondaryFixedVariant,
    /// Mode-independent tertiary fixed color.
    TertiaryFixed,
    /// Dimmed tertiary fixed color.
    TertiaryFixedDim,
    /// Content on tertiary fixed colors.
    OnTertiaryFixed,
    /// Variant content on tertiary fixed colors.
    OnTertiaryFixedVariant,
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
            "surface_dim" => Some(Self::SurfaceDim),
            "surface_bright" => Some(Self::SurfaceBright),
            "surface_container_lowest" => Some(Self::SurfaceContainerLowest),
            "surface_container_low" => Some(Self::SurfaceContainerLow),
            "surface_container" => Some(Self::SurfaceContainer),
            "surface_container_high" => Some(Self::SurfaceContainerHigh),
            "surface_container_highest" => Some(Self::SurfaceContainerHighest),
            "surface_variant" => Some(Self::SurfaceVariant),
            "on_surface_variant" => Some(Self::OnSurfaceVariant),
            "inverse_surface" => Some(Self::InverseSurface),
            "inverse_on_surface" => Some(Self::InverseOnSurface),
            "outline" => Some(Self::Outline),
            "outline_variant" => Some(Self::OutlineVariant),
            "shadow" => Some(Self::Shadow),
            "scrim" => Some(Self::Scrim),
            "surface_tint" => Some(Self::SurfaceTint),
            "inverse_primary" => Some(Self::InversePrimary),
            "primary_fixed" => Some(Self::PrimaryFixed),
            "primary_fixed_dim" => Some(Self::PrimaryFixedDim),
            "on_primary_fixed" => Some(Self::OnPrimaryFixed),
            "on_primary_fixed_variant" => Some(Self::OnPrimaryFixedVariant),
            "secondary_fixed" => Some(Self::SecondaryFixed),
            "secondary_fixed_dim" => Some(Self::SecondaryFixedDim),
            "on_secondary_fixed" => Some(Self::OnSecondaryFixed),
            "on_secondary_fixed_variant" => Some(Self::OnSecondaryFixedVariant),
            "tertiary_fixed" => Some(Self::TertiaryFixed),
            "tertiary_fixed_dim" => Some(Self::TertiaryFixedDim),
            "on_tertiary_fixed" => Some(Self::OnTertiaryFixed),
            "on_tertiary_fixed_variant" => Some(Self::OnTertiaryFixedVariant),
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
#[allow(missing_docs)]
pub struct MaterialColors {
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,

    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,

    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,

    pub background: Color,
    pub on_background: Color,
    pub surface: Color,
    pub on_surface: Color,

    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,

    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,

    pub outline: Color,
    pub outline_variant: Color,
    pub shadow: Color,
    pub scrim: Color,

    pub surface_tint: Color,
    pub inverse_primary: Color,

    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,

    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,

    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed: Color,
    pub on_tertiary_fixed_variant: Color,

    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
}

impl MaterialColors {
    /// Returns the generated value for a Material role.
    #[must_use]
    pub const fn resolve(&self, role: MaterialColor) -> Color {
        match role {
            MaterialColor::Primary => self.primary,
            MaterialColor::OnPrimary => self.on_primary,
            MaterialColor::PrimaryContainer => self.primary_container,
            MaterialColor::OnPrimaryContainer => self.on_primary_container,

            MaterialColor::Secondary => self.secondary,
            MaterialColor::OnSecondary => self.on_secondary,
            MaterialColor::SecondaryContainer => self.secondary_container,
            MaterialColor::OnSecondaryContainer => self.on_secondary_container,

            MaterialColor::Tertiary => self.tertiary,
            MaterialColor::OnTertiary => self.on_tertiary,
            MaterialColor::TertiaryContainer => self.tertiary_container,
            MaterialColor::OnTertiaryContainer => self.on_tertiary_container,

            MaterialColor::Background => self.background,
            MaterialColor::OnBackground => self.on_background,
            MaterialColor::Surface => self.surface,
            MaterialColor::OnSurface => self.on_surface,

            MaterialColor::SurfaceDim => self.surface_dim,
            MaterialColor::SurfaceBright => self.surface_bright,
            MaterialColor::SurfaceContainerLowest => self.surface_container_lowest,
            MaterialColor::SurfaceContainerLow => self.surface_container_low,
            MaterialColor::SurfaceContainer => self.surface_container,
            MaterialColor::SurfaceContainerHigh => self.surface_container_high,
            MaterialColor::SurfaceContainerHighest => self.surface_container_highest,

            MaterialColor::SurfaceVariant => self.surface_variant,
            MaterialColor::OnSurfaceVariant => self.on_surface_variant,
            MaterialColor::InverseSurface => self.inverse_surface,
            MaterialColor::InverseOnSurface => self.inverse_on_surface,

            MaterialColor::Outline => self.outline,
            MaterialColor::OutlineVariant => self.outline_variant,
            MaterialColor::Shadow => self.shadow,
            MaterialColor::Scrim => self.scrim,

            MaterialColor::SurfaceTint => self.surface_tint,
            MaterialColor::InversePrimary => self.inverse_primary,

            MaterialColor::PrimaryFixed => self.primary_fixed,
            MaterialColor::PrimaryFixedDim => self.primary_fixed_dim,
            MaterialColor::OnPrimaryFixed => self.on_primary_fixed,
            MaterialColor::OnPrimaryFixedVariant => self.on_primary_fixed_variant,

            MaterialColor::SecondaryFixed => self.secondary_fixed,
            MaterialColor::SecondaryFixedDim => self.secondary_fixed_dim,
            MaterialColor::OnSecondaryFixed => self.on_secondary_fixed,
            MaterialColor::OnSecondaryFixedVariant => self.on_secondary_fixed_variant,

            MaterialColor::TertiaryFixed => self.tertiary_fixed,
            MaterialColor::TertiaryFixedDim => self.tertiary_fixed_dim,
            MaterialColor::OnTertiaryFixed => self.on_tertiary_fixed,
            MaterialColor::OnTertiaryFixedVariant => self.on_tertiary_fixed_variant,

            MaterialColor::Error => self.error,
            MaterialColor::OnError => self.on_error,
            MaterialColor::ErrorContainer => self.error_container,
            MaterialColor::OnErrorContainer => self.on_error_container,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MaterialToneSet {
    pub(crate) accent: u8,
    pub(crate) on_accent: u8,
    pub(crate) container: u8,
    pub(crate) on_container: u8,
    pub(crate) background: u8,
    pub(crate) on_background: u8,

    pub(crate) surface_dim: u8,
    pub(crate) surface_bright: u8,
    pub(crate) surface_container_lowest: u8,
    pub(crate) surface_container_low: u8,
    pub(crate) surface_container: u8,
    pub(crate) surface_container_high: u8,
    pub(crate) surface_container_highest: u8,
    pub(crate) surface_variant: u8,
    pub(crate) on_surface_variant: u8,
    pub(crate) inverse_surface: u8,
    pub(crate) inverse_on_surface: u8,
    pub(crate) outline: u8,
    pub(crate) outline_variant: u8,
    pub(crate) inverse_primary: u8,
}

impl MaterialToneSet {
    pub(crate) const fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self {
                accent: 40,
                on_accent: 100,
                container: 90,
                on_container: 30,
                background: 98,
                on_background: 10,

                surface_dim: 87,
                surface_bright: 98,
                surface_container_lowest: 100,
                surface_container_low: 96,
                surface_container: 94,
                surface_container_high: 92,
                surface_container_highest: 90,
                surface_variant: 90,
                on_surface_variant: 30,
                inverse_surface: 20,
                inverse_on_surface: 95,
                outline: 50,
                outline_variant: 80,
                inverse_primary: 80,
            },
            ThemeMode::Dark => Self {
                accent: 80,
                on_accent: 20,
                container: 30,
                on_container: 90,
                background: 6,
                on_background: 90,

                surface_dim: 6,
                surface_bright: 24,
                surface_container_lowest: 4,
                surface_container_low: 10,
                surface_container: 12,
                surface_container_high: 17,
                surface_container_highest: 22,
                surface_variant: 30,
                on_surface_variant: 80,
                inverse_surface: 90,
                inverse_on_surface: 20,
                outline: 60,
                outline_variant: 30,
                inverse_primary: 40,
            },
        }
    }
}
