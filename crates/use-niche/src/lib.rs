#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, NicheTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(NicheTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NicheTextError {
    Empty,
}

impl fmt::Display for NicheTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("niche text cannot be empty"),
        }
    }
}

impl Error for NicheTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NicheValueError {
    Negative,
    NonFinite,
}

impl fmt::Display for NicheValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => formatter.write_str("niche value cannot be negative"),
            Self::NonFinite => formatter.write_str("niche value must be finite"),
        }
    }
}

impl Error for NicheValueError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NicheName(String);

impl NicheName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, NicheTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NicheName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NicheName {
    type Err = NicheTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceUse(String);

impl ResourceUse {
    pub fn new(value: impl AsRef<str>) -> Result<Self, NicheTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceUse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ResourceUse {
    type Err = NicheTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct NicheBreadth(f64);

impl NicheBreadth {
    pub fn new(value: f64) -> Result<Self, NicheValueError> {
        if !value.is_finite() {
            return Err(NicheValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(NicheValueError::Negative);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for NicheBreadth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NicheKind {
    Fundamental,
    Realized,
    Trophic,
    Spatial,
    Temporal,
    Functional,
    Unknown,
    Custom(String),
}

impl fmt::Display for NicheKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Fundamental => "fundamental",
            Self::Realized => "realized",
            Self::Trophic => "trophic",
            Self::Spatial => "spatial",
            Self::Temporal => "temporal",
            Self::Functional => "functional",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for NicheKind {
    type Err = NicheKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(NicheKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "fundamental" => Self::Fundamental,
            "realized" => Self::Realized,
            "trophic" => Self::Trophic,
            "spatial" => Self::Spatial,
            "temporal" => Self::Temporal,
            "functional" => Self::Functional,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NicheKindParseError {
    Empty,
}

impl fmt::Display for NicheKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("niche kind cannot be empty"),
        }
    }
}

impl Error for NicheKindParseError {}

#[cfg(test)]
mod tests {
    use super::{NicheBreadth, NicheKind, NicheName, NicheTextError, NicheValueError};

    #[test]
    fn valid_niche_name() -> Result<(), NicheTextError> {
        let name = NicheName::new("canopy pollinator")?;

        assert_eq!(name.as_str(), "canopy pollinator");
        Ok(())
    }

    #[test]
    fn empty_niche_name_rejected() {
        assert_eq!(NicheName::new(""), Err(NicheTextError::Empty));
    }

    #[test]
    fn niche_kind_display_parse() {
        assert_eq!("trophic".parse::<NicheKind>(), Ok(NicheKind::Trophic));
        assert_eq!(NicheKind::Spatial.to_string(), "spatial");
    }

    #[test]
    fn custom_niche_kind() {
        assert_eq!(
            "edge-specialist".parse::<NicheKind>(),
            Ok(NicheKind::Custom("edge-specialist".to_string()))
        );
    }

    #[test]
    fn valid_niche_breadth() -> Result<(), NicheValueError> {
        let breadth = NicheBreadth::new(1.25)?;

        assert_eq!(breadth.get(), 1.25);
        Ok(())
    }

    #[test]
    fn negative_niche_breadth_rejected() {
        assert_eq!(NicheBreadth::new(-0.1), Err(NicheValueError::Negative));
    }
}
