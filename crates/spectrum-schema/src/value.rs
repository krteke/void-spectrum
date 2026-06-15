use core::fmt;
use core::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use spectrum_core::{Color, Length};

use crate::{ColorValueParseError, LengthValueParseError};

/// A validated dot-separated token reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenReference(String);

impl TokenReference {
    /// Creates a token reference from a path without braces.
    pub fn new(path: impl Into<String>) -> Result<Self, ColorValueParseError> {
        let path = path.into();
        let valid = !path.is_empty()
            && path.split('.').all(|segment| {
                !segment.is_empty()
                    && segment.chars().all(|character| {
                        character.is_ascii_alphanumeric() || "_-".contains(character)
                    })
            });
        valid
            .then_some(Self(path))
            .ok_or(ColorValueParseError::InvalidReference)
    }

    /// Returns the referenced token path.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.0
    }
}

/// A direct color or a reference to another color token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorValue {
    /// A concrete color value.
    Literal(Color),
    /// A token reference resolved later.
    Reference(TokenReference),
}

impl FromStr for ColorValue {
    type Err = ColorValueParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(path) = input
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
        {
            return TokenReference::new(path).map(Self::Reference);
        }
        input
            .parse()
            .map(Self::Literal)
            .map_err(ColorValueParseError::InvalidColor)
    }
}

/// A direct length or a reference to another length token.
#[derive(Debug, Clone, PartialEq)]
pub enum LengthValue {
    /// A concrete length value.
    Literal(Length),
    /// A token reference resolved later.
    Reference(TokenReference),
}

impl Eq for LengthValue {}

impl FromStr for LengthValue {
    type Err = LengthValueParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(path) = input
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
        {
            return TokenReference::new(path)
                .map(Self::Reference)
                .map_err(|_| LengthValueParseError::InvalidReference);
        }
        input
            .parse()
            .map(Self::Literal)
            .map_err(LengthValueParseError::InvalidLength)
    }
}

macro_rules! impl_string_value {
    ($name:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Literal(value) => value.fmt(formatter),
                    Self::Reference(reference) => write!(formatter, "{{{}}}", reference.path()),
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

impl_string_value!(ColorValue);
impl_string_value!(LengthValue);
