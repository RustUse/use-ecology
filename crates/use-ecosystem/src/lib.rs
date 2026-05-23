#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, EcosystemTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(EcosystemTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EcosystemTextError {
    Empty,
}

impl fmt::Display for EcosystemTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("ecosystem text cannot be empty"),
        }
    }
}

impl Error for EcosystemTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EcosystemName(String);

impl EcosystemName {
    /// # Errors
    /// Returns `EcosystemTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EcosystemTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EcosystemName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for EcosystemName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EcosystemName {
    type Err = EcosystemTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EcosystemComponent(String);

impl EcosystemComponent {
    /// # Errors
    /// Returns `EcosystemTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EcosystemTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EcosystemComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EcosystemComponent {
    type Err = EcosystemTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EcosystemKind {
    Terrestrial,
    Aquatic,
    Marine,
    Freshwater,
    Forest,
    Grassland,
    Desert,
    Tundra,
    Wetland,
    Urban,
    Agricultural,
    Unknown,
    Custom(String),
}

impl fmt::Display for EcosystemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Terrestrial => "terrestrial",
            Self::Aquatic => "aquatic",
            Self::Marine => "marine",
            Self::Freshwater => "freshwater",
            Self::Forest => "forest",
            Self::Grassland => "grassland",
            Self::Desert => "desert",
            Self::Tundra => "tundra",
            Self::Wetland => "wetland",
            Self::Urban => "urban",
            Self::Agricultural => "agricultural",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for EcosystemKind {
    type Err = EcosystemKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(EcosystemKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "terrestrial" => Self::Terrestrial,
            "aquatic" => Self::Aquatic,
            "marine" => Self::Marine,
            "freshwater" => Self::Freshwater,
            "forest" => Self::Forest,
            "grassland" => Self::Grassland,
            "desert" => Self::Desert,
            "tundra" => Self::Tundra,
            "wetland" => Self::Wetland,
            "urban" => Self::Urban,
            "agricultural" => Self::Agricultural,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EcosystemKindParseError {
    Empty,
}

impl fmt::Display for EcosystemKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("ecosystem kind cannot be empty"),
        }
    }
}

impl Error for EcosystemKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EcosystemScale {
    Micro,
    Local,
    Regional,
    Biome,
    Global,
    Unknown,
    Custom(String),
}

impl fmt::Display for EcosystemScale {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Micro => "micro",
            Self::Local => "local",
            Self::Regional => "regional",
            Self::Biome => "biome",
            Self::Global => "global",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for EcosystemScale {
    type Err = EcosystemScaleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(EcosystemScaleParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "micro" => Self::Micro,
            "local" => Self::Local,
            "regional" => Self::Regional,
            "biome" => Self::Biome,
            "global" => Self::Global,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EcosystemScaleParseError {
    Empty,
}

impl fmt::Display for EcosystemScaleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("ecosystem scale cannot be empty"),
        }
    }
}

impl Error for EcosystemScaleParseError {}

#[cfg(test)]
mod tests {
    use super::{EcosystemKind, EcosystemName, EcosystemScale, EcosystemTextError};

    #[test]
    fn valid_ecosystem_name() -> Result<(), EcosystemTextError> {
        let name = EcosystemName::new("temperate marsh")?;

        assert_eq!(name.as_str(), "temperate marsh");
        assert_eq!(name.to_string(), "temperate marsh");
        Ok(())
    }

    #[test]
    fn empty_ecosystem_name_rejected() {
        assert_eq!(EcosystemName::new("  "), Err(EcosystemTextError::Empty));
    }

    #[test]
    fn ecosystem_kind_display_parse() {
        assert_eq!(
            "wetland".parse::<EcosystemKind>(),
            Ok(EcosystemKind::Wetland)
        );
        assert_eq!(EcosystemKind::Freshwater.to_string(), "freshwater");
    }

    #[test]
    fn ecosystem_scale_display_parse() {
        assert_eq!(
            "regional".parse::<EcosystemScale>(),
            Ok(EcosystemScale::Regional)
        );
        assert_eq!(EcosystemScale::Global.to_string(), "global");
    }

    #[test]
    fn custom_ecosystem_kind() {
        assert_eq!(
            "kelp forest mosaic".parse::<EcosystemKind>(),
            Ok(EcosystemKind::Custom("kelp forest mosaic".to_string()))
        );
    }
}
