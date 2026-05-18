#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, HabitatTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(HabitatTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HabitatTextError {
    Empty,
}

impl fmt::Display for HabitatTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("habitat text cannot be empty"),
        }
    }
}

impl Error for HabitatTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HabitatName(String);

impl HabitatName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, HabitatTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HabitatName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HabitatName {
    type Err = HabitatTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HabitatFeature(String);

impl HabitatFeature {
    pub fn new(value: impl AsRef<str>) -> Result<Self, HabitatTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HabitatFeature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HabitatFeature {
    type Err = HabitatTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HabitatKind {
    Forest,
    Grassland,
    Desert,
    Wetland,
    River,
    Lake,
    Ocean,
    Reef,
    Cave,
    Soil,
    Urban,
    Agricultural,
    Unknown,
    Custom(String),
}

impl fmt::Display for HabitatKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Forest => "forest",
            Self::Grassland => "grassland",
            Self::Desert => "desert",
            Self::Wetland => "wetland",
            Self::River => "river",
            Self::Lake => "lake",
            Self::Ocean => "ocean",
            Self::Reef => "reef",
            Self::Cave => "cave",
            Self::Soil => "soil",
            Self::Urban => "urban",
            Self::Agricultural => "agricultural",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for HabitatKind {
    type Err = HabitatKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HabitatKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "forest" => Self::Forest,
            "grassland" => Self::Grassland,
            "desert" => Self::Desert,
            "wetland" => Self::Wetland,
            "river" => Self::River,
            "lake" => Self::Lake,
            "ocean" => Self::Ocean,
            "reef" => Self::Reef,
            "cave" => Self::Cave,
            "soil" => Self::Soil,
            "urban" => Self::Urban,
            "agricultural" => Self::Agricultural,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HabitatKindParseError {
    Empty,
}

impl fmt::Display for HabitatKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("habitat kind cannot be empty"),
        }
    }
}

impl Error for HabitatKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HabitatCondition {
    Intact,
    Fragmented,
    Degraded,
    Restored,
    Protected,
    Disturbed,
    Unknown,
    Custom(String),
}

impl fmt::Display for HabitatCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Intact => "intact",
            Self::Fragmented => "fragmented",
            Self::Degraded => "degraded",
            Self::Restored => "restored",
            Self::Protected => "protected",
            Self::Disturbed => "disturbed",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for HabitatCondition {
    type Err = HabitatConditionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HabitatConditionParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "intact" => Self::Intact,
            "fragmented" => Self::Fragmented,
            "degraded" => Self::Degraded,
            "restored" => Self::Restored,
            "protected" => Self::Protected,
            "disturbed" => Self::Disturbed,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HabitatConditionParseError {
    Empty,
}

impl fmt::Display for HabitatConditionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("habitat condition cannot be empty"),
        }
    }
}

impl Error for HabitatConditionParseError {}

#[cfg(test)]
mod tests {
    use super::{HabitatCondition, HabitatKind, HabitatName, HabitatTextError};

    #[test]
    fn valid_habitat_name() -> Result<(), HabitatTextError> {
        let name = HabitatName::new("riparian corridor")?;

        assert_eq!(name.as_str(), "riparian corridor");
        Ok(())
    }

    #[test]
    fn empty_habitat_name_rejected() {
        assert_eq!(HabitatName::new(" "), Err(HabitatTextError::Empty));
    }

    #[test]
    fn habitat_kind_display_parse() {
        assert_eq!("reef".parse::<HabitatKind>(), Ok(HabitatKind::Reef));
        assert_eq!(HabitatKind::Agricultural.to_string(), "agricultural");
    }

    #[test]
    fn habitat_condition_display_parse() {
        assert_eq!(
            "protected".parse::<HabitatCondition>(),
            Ok(HabitatCondition::Protected)
        );
        assert_eq!(HabitatCondition::Fragmented.to_string(), "fragmented");
    }

    #[test]
    fn custom_habitat_kind() {
        assert_eq!(
            "tidal-flat".parse::<HabitatKind>(),
            Ok(HabitatKind::Custom("tidal-flat".to_string()))
        );
    }
}
