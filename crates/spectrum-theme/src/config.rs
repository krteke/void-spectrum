//! Contract-aware configuration sources.

use core::str::FromStr;
use std::collections::BTreeSet;

use crate::{
    __private::{MaterialColor, ThemeMode, ThemeValue, TokenSource, material_colors},
    Color, FontStyle, FontWeight, Length, LineHeight, Radius, ShadowLayer, ThemeBuildError,
};

/// Errors produced while loading a TOML contract-aware source.
#[derive(Debug, thiserror::Error)]
pub enum TomlThemeSourceError {
    /// The input is not valid TOML.
    #[error("failed to parse TOML theme source")]
    Parse(#[from] toml::de::Error),
    /// The top-level seed is not a valid color.
    #[error("invalid seed color '{value}'")]
    InvalidSeed {
        /// Invalid seed text.
        value: String,
    },
    /// `meta.mode` is not `light` or `dark`.
    #[error("invalid theme mode '{value}'")]
    InvalidMode {
        /// Invalid mode text.
        value: String,
    },
}

/// A TOML source that reads values by generated token path.
#[derive(Debug, Clone)]
pub struct TomlThemeSource {
    root: toml::Table,
    seed: Option<Color>,
    mode: ThemeMode,
}

impl TomlThemeSource {
    /// Parses a TOML source.
    pub fn parse(input: &str) -> Result<Self, TomlThemeSourceError> {
        input.parse()
    }

    /// Returns a scalar token value as text.
    pub fn token_text(&self, path: &str) -> Result<String, ThemeBuildError> {
        match self.value(path)? {
            toml::Value::String(value) => Ok(value.clone()),
            toml::Value::Integer(value) => Ok(value.to_string()),
            toml::Value::Float(value) => Ok(value.to_string()),
            toml::Value::Boolean(value) => Ok(value.to_string()),
            _ => Err(invalid(path, "expected a scalar token value")),
        }
    }

    fn value(&self, path: &str) -> Result<&toml::Value, ThemeBuildError> {
        if let Some(value) = self.root.get(path) {
            return Ok(value);
        }
        let mut table = &self.root;
        let mut parts = path.split('.').peekable();
        while let Some(part) = parts.next() {
            let Some(value) = table.get(part) else {
                return Err(ThemeBuildError::MissingToken {
                    path: path.to_owned(),
                });
            };
            if parts.peek().is_none() {
                return Ok(value);
            }
            table = value
                .as_table()
                .ok_or_else(|| invalid(path, "expected an intermediate table"))?;
        }
        Err(ThemeBuildError::MissingToken {
            path: path.to_owned(),
        })
    }
}

impl FromStr for TomlThemeSource {
    type Err = TomlThemeSourceError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let root = input.parse::<toml::Table>()?;
        let seed = root
            .get("seed")
            .and_then(toml::Value::as_str)
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| TomlThemeSourceError::InvalidSeed {
                        value: value.to_owned(),
                    })
            })
            .transpose()?;
        let mode = match root
            .get("meta")
            .and_then(toml::Value::as_table)
            .and_then(|meta| meta.get("mode"))
            .and_then(toml::Value::as_str)
        {
            Some("light") => ThemeMode::Light,
            Some("dark") | None => ThemeMode::Dark,
            Some(value) => {
                return Err(TomlThemeSourceError::InvalidMode {
                    value: value.to_owned(),
                });
            }
        };
        Ok(Self { root, seed, mode })
    }
}

impl TokenSource for TomlThemeSource {
    type Error = ThemeBuildError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, ThemeBuildError::MissingToken { .. })
    }
}

impl ThemeValue<TomlThemeSource> for Color {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_color(source, path, &mut BTreeSet::new())
    }
}

impl ThemeValue<TomlThemeSource> for Length {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_scalar(source, path)
    }
}

impl ThemeValue<TomlThemeSource> for Radius {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_scalar(source, path)
    }
}

impl ThemeValue<TomlThemeSource> for FontWeight {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_scalar(source, path)
    }
}

impl ThemeValue<TomlThemeSource> for FontStyle {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_scalar(source, path)
    }
}

impl ThemeValue<TomlThemeSource> for LineHeight {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        read_scalar(source, path)
    }
}

impl ThemeValue<TomlThemeSource> for ShadowLayer {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        let color = source.token(&format!("{path}.color"))?;
        let offset_x = source.token(&format!("{path}.offset_x"))?;
        let offset_y = source.token(&format!("{path}.offset_y"))?;
        let blur = source.token(&format!("{path}.blur"))?;
        let spread = source.token(&format!("{path}.spread"))?;
        Self::new(color, offset_x, offset_y, blur, spread)
            .map_err(|error| invalid(path, error.to_string()))
    }
}

fn read_scalar<T: FromStr>(source: &TomlThemeSource, path: &str) -> Result<T, ThemeBuildError>
where
    T::Err: core::fmt::Display,
{
    source
        .token_text(path)?
        .parse::<T>()
        .map_err(|error| invalid(path, error.to_string()))
}

fn read_color(
    source: &TomlThemeSource,
    path: &str,
    seen: &mut BTreeSet<String>,
) -> Result<Color, ThemeBuildError> {
    if !seen.insert(path.to_owned()) {
        return Err(invalid(path, "cyclic color reference"));
    }
    let value = source.token_text(path)?;
    let Some(reference) = reference(&value) else {
        return value
            .parse::<Color>()
            .map_err(|error| invalid(path, error.to_string()));
    };
    if let Some(role) = reference.strip_prefix("material.") {
        let role = MaterialColor::from_name(role)
            .ok_or_else(|| invalid(path, format!("unknown Material role '{role}'")))?;
        let source_seed = source.seed.ok_or_else(|| ThemeBuildError::MissingSeed {
            path: path.to_owned(),
        })?;
        return Ok(material_colors(source_seed, source.mode, path)?.resolve(role));
    }
    read_color(source, reference, seen)
}

fn reference(value: &str) -> Option<&str> {
    value.strip_prefix('{')?.strip_suffix('}')
}

fn invalid(path: impl Into<String>, message: impl Into<String>) -> ThemeBuildError {
    ThemeBuildError::InvalidTokenValue {
        path: path.into(),
        message: message.into(),
    }
}
