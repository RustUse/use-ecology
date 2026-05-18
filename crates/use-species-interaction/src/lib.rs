#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, InteractionTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(InteractionTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteractionTextError {
    Empty,
}

impl fmt::Display for InteractionTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("interaction text cannot be empty"),
        }
    }
}

impl Error for InteractionTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SpeciesInteractionKind {
    Predation,
    Competition,
    Mutualism,
    Commensalism,
    Parasitism,
    Amensalism,
    Herbivory,
    Symbiosis,
    Neutralism,
    Unknown,
    Custom(String),
}

impl fmt::Display for SpeciesInteractionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Predation => "predation",
            Self::Competition => "competition",
            Self::Mutualism => "mutualism",
            Self::Commensalism => "commensalism",
            Self::Parasitism => "parasitism",
            Self::Amensalism => "amensalism",
            Self::Herbivory => "herbivory",
            Self::Symbiosis => "symbiosis",
            Self::Neutralism => "neutralism",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for SpeciesInteractionKind {
    type Err = SpeciesInteractionKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SpeciesInteractionKindParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "predation" => Self::Predation,
            "competition" => Self::Competition,
            "mutualism" => Self::Mutualism,
            "commensalism" => Self::Commensalism,
            "parasitism" => Self::Parasitism,
            "amensalism" => Self::Amensalism,
            "herbivory" => Self::Herbivory,
            "symbiosis" => Self::Symbiosis,
            "neutralism" => Self::Neutralism,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpeciesInteractionKindParseError {
    Empty,
}

impl fmt::Display for SpeciesInteractionKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("species interaction kind cannot be empty"),
        }
    }
}

impl Error for SpeciesInteractionKindParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InteractionStrength {
    Weak,
    Moderate,
    Strong,
    Unknown,
    Custom(String),
}

impl fmt::Display for InteractionStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Weak => "weak",
            Self::Moderate => "moderate",
            Self::Strong => "strong",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for InteractionStrength {
    type Err = InteractionStrengthParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(InteractionStrengthParseError::Empty);
        }

        Ok(match normalized_token(trimmed).as_str() {
            "weak" => Self::Weak,
            "moderate" => Self::Moderate,
            "strong" => Self::Strong,
            "unknown" => Self::Unknown,
            _ => Self::Custom(trimmed.to_string()),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteractionStrengthParseError {
    Empty,
}

impl fmt::Display for InteractionStrengthParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("interaction strength cannot be empty"),
        }
    }
}

impl Error for InteractionStrengthParseError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpeciesInteraction {
    first: String,
    second: String,
    kind: SpeciesInteractionKind,
    strength: Option<InteractionStrength>,
}

impl SpeciesInteraction {
    pub fn new(
        first: impl AsRef<str>,
        second: impl AsRef<str>,
        kind: SpeciesInteractionKind,
    ) -> Result<Self, InteractionTextError> {
        Ok(Self {
            first: non_empty_text(first)?,
            second: non_empty_text(second)?,
            kind,
            strength: None,
        })
    }

    #[must_use]
    pub fn with_strength(mut self, strength: InteractionStrength) -> Self {
        self.strength = Some(strength);
        self
    }

    #[must_use]
    pub fn first(&self) -> &str {
        &self.first
    }

    #[must_use]
    pub fn second(&self) -> &str {
        &self.second
    }

    #[must_use]
    pub const fn kind(&self) -> &SpeciesInteractionKind {
        &self.kind
    }

    #[must_use]
    pub const fn strength(&self) -> Option<&InteractionStrength> {
        self.strength.as_ref()
    }
}

impl fmt::Display for SpeciesInteraction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -[{}]-> {}",
            self.first, self.kind, self.second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InteractionStrength, InteractionTextError, SpeciesInteraction, SpeciesInteractionKind,
    };

    #[test]
    fn interaction_kind_display_parse() {
        assert_eq!(
            "mutualism".parse::<SpeciesInteractionKind>(),
            Ok(SpeciesInteractionKind::Mutualism)
        );
        assert_eq!(SpeciesInteractionKind::Predation.to_string(), "predation");
    }

    #[test]
    fn custom_interaction_kind() {
        assert_eq!(
            "facilitation".parse::<SpeciesInteractionKind>(),
            Ok(SpeciesInteractionKind::Custom("facilitation".to_string()))
        );
    }

    #[test]
    fn interaction_strength_display_parse() {
        assert_eq!(
            "strong".parse::<InteractionStrength>(),
            Ok(InteractionStrength::Strong)
        );
        assert_eq!(InteractionStrength::Moderate.to_string(), "moderate");
    }

    #[test]
    fn species_interaction_construction() -> Result<(), InteractionTextError> {
        let interaction =
            SpeciesInteraction::new("coral", "algae", SpeciesInteractionKind::Mutualism)?
                .with_strength(InteractionStrength::Strong);

        assert_eq!(interaction.first(), "coral");
        assert_eq!(interaction.second(), "algae");
        assert_eq!(interaction.kind(), &SpeciesInteractionKind::Mutualism);
        assert_eq!(interaction.strength(), Some(&InteractionStrength::Strong));
        Ok(())
    }
}
