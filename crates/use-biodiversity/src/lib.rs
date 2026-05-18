#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiodiversityValueError {
    Negative,
    NonFinite,
}

impl fmt::Display for BiodiversityValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => formatter.write_str("biodiversity value cannot be negative"),
            Self::NonFinite => formatter.write_str("biodiversity value must be finite"),
        }
    }
}

impl Error for BiodiversityValueError {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpeciesRichness(u64);

impl SpeciesRichness {
    pub fn new(value: i64) -> Result<Self, BiodiversityValueError> {
        if value < 0 {
            Err(BiodiversityValueError::Negative)
        } else {
            Ok(Self(value as u64))
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for SpeciesRichness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiversityIndexKind {
    SpeciesRichness,
    Shannon,
    Simpson,
    Evenness,
    BetaDiversity,
    PhylogeneticDiversity,
    Unknown,
    Custom(String),
}

impl fmt::Display for DiversityIndexKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::SpeciesRichness => "species-richness",
            Self::Shannon => "shannon",
            Self::Simpson => "simpson",
            Self::Evenness => "evenness",
            Self::BetaDiversity => "beta-diversity",
            Self::PhylogeneticDiversity => "phylogenetic-diversity",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for DiversityIndexKind {
    type Err = DiversityIndexKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DiversityIndexKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "species-richness" => Self::SpeciesRichness,
            "shannon" => Self::Shannon,
            "simpson" => Self::Simpson,
            "evenness" => Self::Evenness,
            "beta-diversity" => Self::BetaDiversity,
            "phylogenetic-diversity" => Self::PhylogeneticDiversity,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiversityIndexKindParseError {
    Empty,
}

impl fmt::Display for DiversityIndexKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diversity index kind cannot be empty"),
        }
    }
}

impl Error for DiversityIndexKindParseError {}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DiversityIndex {
    kind: DiversityIndexKind,
    value: f64,
}

impl DiversityIndex {
    pub fn new(kind: DiversityIndexKind, value: f64) -> Result<Self, BiodiversityValueError> {
        if !value.is_finite() {
            return Err(BiodiversityValueError::NonFinite);
        }

        Ok(Self { kind, value })
    }

    #[must_use]
    pub const fn kind(&self) -> &DiversityIndexKind {
        &self.kind
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for DiversityIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.kind, self.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BiodiversityMeasure {
    SpeciesRichness(SpeciesRichness),
    DiversityIndex(DiversityIndex),
}

impl fmt::Display for BiodiversityMeasure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpeciesRichness(value) => write!(formatter, "species-richness: {}", value),
            Self::DiversityIndex(value) => value.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BiodiversityMeasure, BiodiversityValueError, DiversityIndex, DiversityIndexKind,
        SpeciesRichness,
    };

    #[test]
    fn valid_species_richness() -> Result<(), BiodiversityValueError> {
        let richness = SpeciesRichness::new(12)?;

        assert_eq!(richness.get(), 12);
        Ok(())
    }

    #[test]
    fn negative_species_richness_rejected() {
        assert_eq!(
            SpeciesRichness::new(-1),
            Err(BiodiversityValueError::Negative)
        );
    }

    #[test]
    fn diversity_index_kind_display_parse() {
        assert_eq!(
            "shannon".parse::<DiversityIndexKind>(),
            Ok(DiversityIndexKind::Shannon)
        );
        assert_eq!(
            DiversityIndexKind::BetaDiversity.to_string(),
            "beta-diversity"
        );
    }

    #[test]
    fn custom_diversity_index_kind() {
        assert_eq!(
            "functional-diversity".parse::<DiversityIndexKind>(),
            Ok(DiversityIndexKind::Custom(
                "functional-diversity".to_string()
            ))
        );
    }

    #[test]
    fn diversity_index_construction() -> Result<(), BiodiversityValueError> {
        let index = DiversityIndex::new(DiversityIndexKind::Shannon, 2.3)?;

        assert_eq!(index.kind(), &DiversityIndexKind::Shannon);
        assert_eq!(index.value(), 2.3);
        assert_eq!(
            BiodiversityMeasure::DiversityIndex(index).to_string(),
            "shannon: 2.3"
        );
        Ok(())
    }
}
