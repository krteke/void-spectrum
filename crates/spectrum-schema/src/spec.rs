use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use spectrum_core::Color;

use crate::{ColorValue, LengthValue, ThemeMeta};

/// A partially specified theme configuration.
///
/// ```
/// use spectrum_core::Color;
/// use spectrum_schema::ThemeSpec;
///
/// let spec = ThemeSpec::new("Midnight").with_seed(Color::new(80, 120, 200));
/// assert_eq!(spec.seed, Some(Color::new(80, 120, 200)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeSpec {
    /// Human-readable theme metadata.
    pub meta: ThemeMeta,
    /// Optional brand color used to derive a palette.
    #[serde(default, with = "optional_color")]
    pub seed: Option<Color>,
    /// Color token overrides keyed by token path.
    #[serde(default)]
    pub colors: BTreeMap<String, ColorValue>,
    /// Length token overrides keyed by token path.
    #[serde(default)]
    pub lengths: BTreeMap<String, LengthValue>,
}

impl ThemeSpec {
    /// Creates a theme specification without a seed color.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            meta: ThemeMeta::new(name),
            seed: None,
            colors: BTreeMap::new(),
            lengths: BTreeMap::new(),
        }
    }

    /// Sets the seed color.
    #[must_use]
    pub const fn with_seed(mut self, seed: Color) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Adds or replaces a color token override.
    #[must_use]
    pub fn with_color(mut self, path: impl Into<String>, value: ColorValue) -> Self {
        self.colors.insert(path.into(), value);
        self
    }

    /// Adds or replaces a length token override.
    #[must_use]
    pub fn with_length(mut self, path: impl Into<String>, value: LengthValue) -> Self {
        self.lengths.insert(path.into(), value);
        self
    }
}

mod optional_color {
    use serde::{Deserialize, Deserializer, Serializer};
    use spectrum_core::Color;

    #[allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(color) => serializer.serialize_some(&color.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|value| value.parse().map_err(serde::de::Error::custom))
            .transpose()
    }
}
