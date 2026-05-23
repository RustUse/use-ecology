#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, ConservationTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(ConservationTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConservationTextError {
    Empty,
}

impl fmt::Display for ConservationTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("conservation text cannot be empty"),
        }
    }
}

impl Error for ConservationTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConservationStatus {
    LeastConcern,
    NearThreatened,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInTheWild,
    Extinct,
    DataDeficient,
    NotEvaluated,
    Unknown,
    Custom(String),
}

impl fmt::Display for ConservationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::LeastConcern => "least-concern",
            Self::NearThreatened => "near-threatened",
            Self::Vulnerable => "vulnerable",
            Self::Endangered => "endangered",
            Self::CriticallyEndangered => "critically-endangered",
            Self::ExtinctInTheWild => "extinct-in-the-wild",
            Self::Extinct => "extinct",
            Self::DataDeficient => "data-deficient",
            Self::NotEvaluated => "not-evaluated",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ConservationStatus {
    type Err = ConservationStatusParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ConservationStatusParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "least-concern" => Self::LeastConcern,
            "near-threatened" => Self::NearThreatened,
            "vulnerable" => Self::Vulnerable,
            "endangered" => Self::Endangered,
            "critically-endangered" => Self::CriticallyEndangered,
            "extinct-in-the-wild" => Self::ExtinctInTheWild,
            "extinct" => Self::Extinct,
            "data-deficient" => Self::DataDeficient,
            "not-evaluated" => Self::NotEvaluated,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConservationStatusParseError {
    Empty,
}

impl fmt::Display for ConservationStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("conservation status cannot be empty"),
        }
    }
}

impl Error for ConservationStatusParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ThreatKind {
    HabitatLoss,
    ClimateChange,
    Pollution,
    InvasiveSpecies,
    Overexploitation,
    Disease,
    Fragmentation,
    Unknown,
    Custom(String),
}

impl fmt::Display for ThreatKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::HabitatLoss => "habitat-loss",
            Self::ClimateChange => "climate-change",
            Self::Pollution => "pollution",
            Self::InvasiveSpecies => "invasive-species",
            Self::Overexploitation => "overexploitation",
            Self::Disease => "disease",
            Self::Fragmentation => "fragmentation",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ThreatKind {
    type Err = ThreatKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ThreatKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "habitat-loss" => Self::HabitatLoss,
            "climate-change" => Self::ClimateChange,
            "pollution" => Self::Pollution,
            "invasive-species" => Self::InvasiveSpecies,
            "overexploitation" => Self::Overexploitation,
            "disease" => Self::Disease,
            "fragmentation" => Self::Fragmentation,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreatKindParseError {
    Empty,
}

impl fmt::Display for ThreatKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("threat kind cannot be empty"),
        }
    }
}

impl Error for ThreatKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConservationStatusSystem(String);

impl ConservationStatusSystem {
    /// # Errors
    /// Returns `ConservationTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConservationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConservationStatusSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProtectionStatus(String);

impl ProtectionStatus {
    /// # Errors
    /// Returns `ConservationTextError::Empty` when `value` is blank.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConservationTextError> {
        non_empty_text(value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProtectionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConservationStatus, ConservationStatusSystem, ConservationTextError, ProtectionStatus,
        ThreatKind,
    };

    #[test]
    fn conservation_status_display_parse() {
        assert_eq!(
            "vulnerable".parse::<ConservationStatus>(),
            Ok(ConservationStatus::Vulnerable)
        );
        assert_eq!(ConservationStatus::Endangered.to_string(), "endangered");
    }

    #[test]
    fn custom_conservation_status() {
        assert_eq!(
            "regionally-endemic".parse::<ConservationStatus>(),
            Ok(ConservationStatus::Custom("regionally-endemic".to_string()))
        );
    }

    #[test]
    fn threat_kind_display_parse() {
        assert_eq!(
            "habitat-loss".parse::<ThreatKind>(),
            Ok(ThreatKind::HabitatLoss)
        );
        assert_eq!(ThreatKind::ClimateChange.to_string(), "climate-change");
    }

    #[test]
    fn protection_status_construction() -> Result<(), ConservationTextError> {
        let protection = ProtectionStatus::new("protected-area")?;

        assert_eq!(protection.to_string(), "protected-area");
        Ok(())
    }

    #[test]
    fn conservation_status_system_construction() -> Result<(), ConservationTextError> {
        let system = ConservationStatusSystem::new("IUCN")?;

        assert_eq!(system.to_string(), "IUCN");
        Ok(())
    }
}
