#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FoodWebTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(FoodWebTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FoodWebTextError {
    Empty,
}

impl fmt::Display for FoodWebTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("food web text cannot be empty"),
        }
    }
}

impl Error for FoodWebTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FoodWebName(String);

impl FoodWebName {
    /// # Errors
    /// Returns `FoodWebTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FoodWebTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FoodWebName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FoodWebName {
    type Err = FoodWebTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FeedingRelation {
    Consumes,
    PreysOn,
    Grazes,
    Parasitizes,
    Scavenges,
    Decomposes,
    Unknown,
    Custom(String),
}

impl fmt::Display for FeedingRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Consumes => "consumes",
            Self::PreysOn => "preys-on",
            Self::Grazes => "grazes",
            Self::Parasitizes => "parasitizes",
            Self::Scavenges => "scavenges",
            Self::Decomposes => "decomposes",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for FeedingRelation {
    type Err = FeedingRelationParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FeedingRelationParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "consumes" => Self::Consumes,
            "preys-on" => Self::PreysOn,
            "grazes" => Self::Grazes,
            "parasitizes" => Self::Parasitizes,
            "scavenges" => Self::Scavenges,
            "decomposes" => Self::Decomposes,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeedingRelationParseError {
    Empty,
}

impl fmt::Display for FeedingRelationParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("feeding relation cannot be empty"),
        }
    }
}

impl Error for FeedingRelationParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnergyFlowDirection {
    ProducerToConsumer,
    PreyToPredator,
    DetritusToDecomposer,
    Unknown,
    Custom(String),
}

impl fmt::Display for EnergyFlowDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ProducerToConsumer => "producer-to-consumer",
            Self::PreyToPredator => "prey-to-predator",
            Self::DetritusToDecomposer => "detritus-to-decomposer",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for EnergyFlowDirection {
    type Err = EnergyFlowDirectionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(EnergyFlowDirectionParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "producer-to-consumer" => Self::ProducerToConsumer,
            "prey-to-predator" => Self::PreyToPredator,
            "detritus-to-decomposer" => Self::DetritusToDecomposer,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnergyFlowDirectionParseError {
    Empty,
}

impl fmt::Display for EnergyFlowDirectionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("energy flow direction cannot be empty"),
        }
    }
}

impl Error for EnergyFlowDirectionParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FoodWebLink {
    source: String,
    target: String,
    relation: FeedingRelation,
}

impl FoodWebLink {
    /// # Errors
    /// Returns `FoodWebTextError::Empty` when `source` or `target` is blank.
    pub fn new(
        source: impl AsRef<str>,
        target: impl AsRef<str>,
        relation: FeedingRelation,
    ) -> Result<Self, FoodWebTextError> {
        Ok(Self {
            source: non_empty_text(source)?,
            target: non_empty_text(target)?,
            relation,
        })
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    #[must_use]
    pub const fn relation(&self) -> &FeedingRelation {
        &self.relation
    }
}

impl fmt::Display for FoodWebLink {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -[{}]-> {}",
            self.source, self.relation, self.target
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{EnergyFlowDirection, FeedingRelation, FoodWebLink, FoodWebName, FoodWebTextError};

    #[test]
    fn valid_food_web_name() -> Result<(), FoodWebTextError> {
        let name = FoodWebName::new("shelf web")?;

        assert_eq!(name.as_str(), "shelf web");
        Ok(())
    }

    #[test]
    fn empty_food_web_name_rejected() {
        assert_eq!(FoodWebName::new(""), Err(FoodWebTextError::Empty));
    }

    #[test]
    fn feeding_relation_display_parse() {
        assert_eq!(
            "consumes".parse::<FeedingRelation>(),
            Ok(FeedingRelation::Consumes)
        );
        assert_eq!(FeedingRelation::PreysOn.to_string(), "preys-on");
    }

    #[test]
    fn energy_flow_direction_display_parse() {
        assert_eq!(
            "prey-to-predator".parse::<EnergyFlowDirection>(),
            Ok(EnergyFlowDirection::PreyToPredator)
        );
        assert_eq!(
            EnergyFlowDirection::DetritusToDecomposer.to_string(),
            "detritus-to-decomposer"
        );
    }

    #[test]
    fn food_web_link_construction() -> Result<(), FoodWebTextError> {
        let link = FoodWebLink::new("zooplankton", "anchovy", FeedingRelation::Consumes)?;

        assert_eq!(link.source(), "zooplankton");
        assert_eq!(link.target(), "anchovy");
        assert_eq!(link.to_string(), "zooplankton -[consumes]-> anchovy");
        Ok(())
    }
}
