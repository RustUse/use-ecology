#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{collections::BTreeSet, error::Error};

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, CommunityTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(CommunityTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommunityTextError {
    Empty,
}

impl fmt::Display for CommunityTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("community text cannot be empty"),
        }
    }
}

impl Error for CommunityTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommunityName(String);

impl CommunityName {
    /// # Errors
    /// Returns `CommunityTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CommunityTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommunityName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CommunityName {
    type Err = CommunityTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunityComposition {
    labels: BTreeSet<String>,
}

impl CommunityComposition {
    /// # Errors
    /// Returns `CommunityTextError::Empty` when any provided label is blank.
    pub fn new<I, S>(labels: I) -> Result<Self, CommunityTextError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut values = BTreeSet::new();

        for label in labels {
            values.insert(non_empty_text(label)?);
        }

        Ok(Self { labels: values })
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.labels.iter()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommunityKind {
    PlantCommunity,
    AnimalCommunity,
    MicrobialCommunity,
    AquaticCommunity,
    TerrestrialCommunity,
    Mixed,
    Unknown,
    Custom(String),
}

impl fmt::Display for CommunityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PlantCommunity => "plant-community",
            Self::AnimalCommunity => "animal-community",
            Self::MicrobialCommunity => "microbial-community",
            Self::AquaticCommunity => "aquatic-community",
            Self::TerrestrialCommunity => "terrestrial-community",
            Self::Mixed => "mixed",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for CommunityKind {
    type Err = CommunityKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CommunityKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "plant-community" => Self::PlantCommunity,
            "animal-community" => Self::AnimalCommunity,
            "microbial-community" => Self::MicrobialCommunity,
            "aquatic-community" => Self::AquaticCommunity,
            "terrestrial-community" => Self::TerrestrialCommunity,
            "mixed" => Self::Mixed,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommunityKindParseError {
    Empty,
}

impl fmt::Display for CommunityKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("community kind cannot be empty"),
        }
    }
}

impl Error for CommunityKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommunityRole {
    Dominant,
    Keystone,
    Foundation,
    Rare,
    Invasive,
    Native,
    Unknown,
    Custom(String),
}

impl fmt::Display for CommunityRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Dominant => "dominant",
            Self::Keystone => "keystone",
            Self::Foundation => "foundation",
            Self::Rare => "rare",
            Self::Invasive => "invasive",
            Self::Native => "native",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for CommunityRole {
    type Err = CommunityRoleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CommunityRoleParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "dominant" => Self::Dominant,
            "keystone" => Self::Keystone,
            "foundation" => Self::Foundation,
            "rare" => Self::Rare,
            "invasive" => Self::Invasive,
            "native" => Self::Native,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommunityRoleParseError {
    Empty,
}

impl fmt::Display for CommunityRoleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("community role cannot be empty"),
        }
    }
}

impl Error for CommunityRoleParseError {}

#[cfg(test)]
mod tests {
    use super::{
        CommunityComposition, CommunityKind, CommunityName, CommunityRole, CommunityTextError,
    };

    #[test]
    fn valid_community_name() -> Result<(), CommunityTextError> {
        let name = CommunityName::new("prairie pollinators")?;

        assert_eq!(name.as_str(), "prairie pollinators");
        Ok(())
    }

    #[test]
    fn empty_community_name_rejected() {
        assert_eq!(CommunityName::new("  "), Err(CommunityTextError::Empty));
    }

    #[test]
    fn community_kind_display_parse() {
        assert_eq!(
            "aquatic-community".parse::<CommunityKind>(),
            Ok(CommunityKind::AquaticCommunity)
        );
        assert_eq!(CommunityKind::Mixed.to_string(), "mixed");
    }

    #[test]
    fn community_role_display_parse() {
        assert_eq!(
            "keystone".parse::<CommunityRole>(),
            Ok(CommunityRole::Keystone)
        );
        assert_eq!(CommunityRole::Native.to_string(), "native");
    }

    #[test]
    fn deterministic_composition_ordering() -> Result<(), CommunityTextError> {
        let composition = CommunityComposition::new(["zebra", "antelope", "buffalo"])?;
        let ordered = composition.iter().map(String::as_str).collect::<Vec<_>>();

        assert_eq!(ordered, vec!["antelope", "buffalo", "zebra"]);
        Ok(())
    }
}
