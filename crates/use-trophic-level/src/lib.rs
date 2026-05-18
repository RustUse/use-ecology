#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, TrophicTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(TrophicTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrophicTextError {
    Empty,
}

impl fmt::Display for TrophicTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("trophic text cannot be empty"),
        }
    }
}

impl Error for TrophicTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrophicValueError {
    Negative,
    NonFinite,
}

impl fmt::Display for TrophicValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => formatter.write_str("trophic value cannot be negative"),
            Self::NonFinite => formatter.write_str("trophic value must be finite"),
        }
    }
}

impl Error for TrophicValueError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TrophicLevel {
    PrimaryProducer,
    PrimaryConsumer,
    SecondaryConsumer,
    TertiaryConsumer,
    QuaternaryConsumer,
    Decomposer,
    Detritivore,
    Omnivore,
    Unknown,
    Custom(String),
}

impl fmt::Display for TrophicLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PrimaryProducer => "primary-producer",
            Self::PrimaryConsumer => "primary-consumer",
            Self::SecondaryConsumer => "secondary-consumer",
            Self::TertiaryConsumer => "tertiary-consumer",
            Self::QuaternaryConsumer => "quaternary-consumer",
            Self::Decomposer => "decomposer",
            Self::Detritivore => "detritivore",
            Self::Omnivore => "omnivore",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for TrophicLevel {
    type Err = TrophicLevelParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TrophicLevelParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "primary-producer" => Self::PrimaryProducer,
            "primary-consumer" => Self::PrimaryConsumer,
            "secondary-consumer" => Self::SecondaryConsumer,
            "tertiary-consumer" => Self::TertiaryConsumer,
            "quaternary-consumer" => Self::QuaternaryConsumer,
            "decomposer" => Self::Decomposer,
            "detritivore" => Self::Detritivore,
            "omnivore" => Self::Omnivore,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrophicLevelParseError {
    Empty,
}

impl fmt::Display for TrophicLevelParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("trophic level cannot be empty"),
        }
    }
}

impl Error for TrophicLevelParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrophicRole(String);

impl TrophicRole {
    pub fn new(value: impl AsRef<str>) -> Result<Self, TrophicTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrophicRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TrophicRole {
    type Err = TrophicTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TrophicPosition(f64);

impl TrophicPosition {
    pub fn new(value: f64) -> Result<Self, TrophicValueError> {
        if !value.is_finite() {
            return Err(TrophicValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(TrophicValueError::Negative);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for TrophicPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{TrophicLevel, TrophicPosition, TrophicRole, TrophicTextError, TrophicValueError};

    #[test]
    fn trophic_level_display_parse() {
        assert_eq!(
            "primary-consumer".parse::<TrophicLevel>(),
            Ok(TrophicLevel::PrimaryConsumer)
        );
        assert_eq!(TrophicLevel::Decomposer.to_string(), "decomposer");
    }

    #[test]
    fn custom_trophic_level() {
        assert_eq!(
            "suspension-feeder".parse::<TrophicLevel>(),
            Ok(TrophicLevel::Custom("suspension-feeder".to_string()))
        );
    }

    #[test]
    fn valid_trophic_position() -> Result<(), TrophicValueError> {
        let position = TrophicPosition::new(2.5)?;

        assert_eq!(position.get(), 2.5);
        Ok(())
    }

    #[test]
    fn negative_trophic_position_rejected() {
        assert_eq!(TrophicPosition::new(-1.0), Err(TrophicValueError::Negative));
    }

    #[test]
    fn trophic_role_construction() -> Result<(), TrophicTextError> {
        let role = TrophicRole::new("reef grazer")?;

        assert_eq!(role.to_string(), "reef grazer");
        Ok(())
    }
}
