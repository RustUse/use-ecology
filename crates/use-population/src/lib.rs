#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, PopulationTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(PopulationTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopulationTextError {
    Empty,
}

impl fmt::Display for PopulationTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("population text cannot be empty"),
        }
    }
}

impl Error for PopulationTextError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopulationValueError {
    Negative,
    NonFinite,
}

impl fmt::Display for PopulationValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => formatter.write_str("population value cannot be negative"),
            Self::NonFinite => formatter.write_str("population value must be finite"),
        }
    }
}

impl Error for PopulationValueError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PopulationId(String);

impl PopulationId {
    /// # Errors
    /// Returns `PopulationTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PopulationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PopulationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PopulationId {
    type Err = PopulationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PopulationName(String);

impl PopulationName {
    /// # Errors
    /// Returns `PopulationTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PopulationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PopulationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PopulationName {
    type Err = PopulationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PopulationSize(u64);

impl PopulationSize {
    /// # Errors
    /// Returns `PopulationValueError::Negative` when `value` is less than zero.
    pub const fn new(value: i64) -> Result<Self, PopulationValueError> {
        if value < 0 {
            Err(PopulationValueError::Negative)
        } else {
            Ok(Self(value.cast_unsigned()))
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for PopulationSize {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PopulationDensity(f64);

impl PopulationDensity {
    /// # Errors
    /// Returns `PopulationValueError::NonFinite` when `value` is not finite.
    /// Returns `PopulationValueError::Negative` when `value` is less than zero.
    pub fn new(value: f64) -> Result<Self, PopulationValueError> {
        if !value.is_finite() {
            return Err(PopulationValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(PopulationValueError::Negative);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for PopulationDensity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PopulationTrend {
    Increasing,
    Decreasing,
    Stable,
    Fluctuating,
    Unknown,
    Custom(String),
}

impl fmt::Display for PopulationTrend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Increasing => "increasing",
            Self::Decreasing => "decreasing",
            Self::Stable => "stable",
            Self::Fluctuating => "fluctuating",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for PopulationTrend {
    type Err = PopulationTrendParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PopulationTrendParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "increasing" => Self::Increasing,
            "decreasing" => Self::Decreasing,
            "stable" => Self::Stable,
            "fluctuating" => Self::Fluctuating,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopulationTrendParseError {
    Empty,
}

impl fmt::Display for PopulationTrendParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("population trend cannot be empty"),
        }
    }
}

impl Error for PopulationTrendParseError {}

#[cfg(test)]
mod tests {
    use super::{
        PopulationDensity, PopulationId, PopulationSize, PopulationTextError, PopulationTrend,
        PopulationValueError,
    };

    #[test]
    fn valid_population_id() -> Result<(), PopulationTextError> {
        let id = PopulationId::new("delta-cranes")?;

        assert_eq!(id.as_str(), "delta-cranes");
        Ok(())
    }

    #[test]
    fn empty_population_id_rejected() {
        assert_eq!(PopulationId::new("  "), Err(PopulationTextError::Empty));
    }

    #[test]
    fn valid_population_size() -> Result<(), PopulationValueError> {
        let size = PopulationSize::new(42)?;

        assert_eq!(size.get(), 42);
        Ok(())
    }

    #[test]
    fn negative_population_size_rejected() {
        assert_eq!(PopulationSize::new(-1), Err(PopulationValueError::Negative));
    }

    #[test]
    fn population_trend_display_parse() {
        assert_eq!(
            "stable".parse::<PopulationTrend>(),
            Ok(PopulationTrend::Stable)
        );
        assert_eq!(PopulationTrend::Fluctuating.to_string(), "fluctuating");
    }

    #[test]
    fn valid_population_density() -> Result<(), PopulationValueError> {
        let density = PopulationDensity::new(3.5)?;

        assert!((density.get() - 3.5).abs() < f64::EPSILON);
        Ok(())
    }
}
