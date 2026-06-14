use serde::{Deserialize, Serialize};
use spectrum_core::ThemeMode;

/// Human-readable metadata attached to a theme specification.
///
/// ```
/// use spectrum_schema::{ThemeMeta, ThemeMode};
///
/// let meta = ThemeMeta::new("Midnight");
/// assert_eq!(meta.mode, ThemeMode::Dark);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeMeta {
    /// Display name of the theme.
    pub name: String,
    /// Optional theme author.
    #[serde(default)]
    pub author: Option<String>,
    /// Target surface mode.
    #[serde(default)]
    pub mode: ThemeMode,
    /// Optional theme format or release version.
    #[serde(default)]
    pub version: Option<String>,
    /// Optional human-readable description.
    #[serde(default)]
    pub description: Option<String>,
}

impl ThemeMeta {
    /// Creates metadata with a name and dark mode defaults.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            author: None,
            mode: ThemeMode::default(),
            version: None,
            description: None,
        }
    }
}
