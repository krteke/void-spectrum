use serde::{Deserialize, Serialize};
use spectrum_core::{Color, Length};

/// A complete shadow layer associated with a typed token path.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShadowSpec {
    /// Token path receiving this layer.
    pub path: String,
    /// Shadow color.
    #[serde(with = "color")]
    pub color: Color,
    /// Horizontal offset.
    #[serde(with = "length")]
    pub offset_x: Length,
    /// Vertical offset.
    #[serde(with = "length")]
    pub offset_y: Length,
    /// Non-negative blur radius.
    #[serde(with = "length")]
    pub blur: Length,
    /// Shadow spread.
    #[serde(with = "length")]
    pub spread: Length,
}

mod color {
    use serde::{Deserialize, Deserializer, Serializer};
    use spectrum_core::Color;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

mod length {
    use serde::{Deserialize, Deserializer, Serializer};
    use spectrum_core::Length;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
