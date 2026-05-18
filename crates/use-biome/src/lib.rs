#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, BiomeTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(BiomeTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiomeTextError {
    Empty,
}

impl fmt::Display for BiomeTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("biome text cannot be empty"),
        }
    }
}

impl Error for BiomeTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BiomeName(String);

impl BiomeName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, BiomeTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BiomeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BiomeName {
    type Err = BiomeTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BiomeClimate(String);

impl BiomeClimate {
    pub fn new(value: impl AsRef<str>) -> Result<Self, BiomeTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BiomeClimate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BiomeClimate {
    type Err = BiomeTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BiomeKind {
    TropicalRainforest,
    TemperateForest,
    BorealForest,
    Grassland,
    Savanna,
    Desert,
    Tundra,
    Mediterranean,
    Wetland,
    Freshwater,
    Marine,
    Unknown,
    Custom(String),
}

impl fmt::Display for BiomeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TropicalRainforest => "tropical-rainforest",
            Self::TemperateForest => "temperate-forest",
            Self::BorealForest => "boreal-forest",
            Self::Grassland => "grassland",
            Self::Savanna => "savanna",
            Self::Desert => "desert",
            Self::Tundra => "tundra",
            Self::Mediterranean => "mediterranean",
            Self::Wetland => "wetland",
            Self::Freshwater => "freshwater",
            Self::Marine => "marine",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for BiomeKind {
    type Err = BiomeKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(BiomeKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "tropical-rainforest" => Self::TropicalRainforest,
            "temperate-forest" => Self::TemperateForest,
            "boreal-forest" => Self::BorealForest,
            "grassland" => Self::Grassland,
            "savanna" => Self::Savanna,
            "desert" => Self::Desert,
            "tundra" => Self::Tundra,
            "mediterranean" => Self::Mediterranean,
            "wetland" => Self::Wetland,
            "freshwater" => Self::Freshwater,
            "marine" => Self::Marine,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiomeKindParseError {
    Empty,
}

impl fmt::Display for BiomeKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("biome kind cannot be empty"),
        }
    }
}

impl Error for BiomeKindParseError {}

#[cfg(test)]
mod tests {
    use super::{BiomeClimate, BiomeKind, BiomeName, BiomeTextError};

    #[test]
    fn valid_biome_name() -> Result<(), BiomeTextError> {
        let name = BiomeName::new("coastal upwelling")?;

        assert_eq!(name.as_str(), "coastal upwelling");
        Ok(())
    }

    #[test]
    fn empty_biome_name_rejected() {
        assert_eq!(BiomeName::new("  "), Err(BiomeTextError::Empty));
    }

    #[test]
    fn biome_kind_display_parse() {
        assert_eq!("marine".parse::<BiomeKind>(), Ok(BiomeKind::Marine));
        assert_eq!(BiomeKind::TemperateForest.to_string(), "temperate-forest");
    }

    #[test]
    fn custom_biome_kind() {
        assert_eq!(
            "fog-desert".parse::<BiomeKind>(),
            Ok(BiomeKind::Custom("fog-desert".to_string()))
        );
    }

    #[test]
    fn biome_climate_construction() -> Result<(), BiomeTextError> {
        let climate = BiomeClimate::new("cool and wet")?;

        assert_eq!(climate.to_string(), "cool and wet");
        Ok(())
    }
}
