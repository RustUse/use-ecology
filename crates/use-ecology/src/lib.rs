#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_biodiversity as biodiversity;
pub use use_biome as biome;
pub use use_community as community;
pub use use_conservation_status as conservation_status;
pub use use_ecosystem as ecosystem;
pub use use_food_web as food_web;
pub use use_habitat as habitat;
pub use use_niche as niche;
pub use use_population as population;
pub use use_species_interaction as species_interaction;
pub use use_trophic_level as trophic_level;

/// Common primitive ecology vocabulary reexports.
pub mod prelude {
    pub use use_biodiversity::{
        BiodiversityMeasure, BiodiversityValueError, DiversityIndex, DiversityIndexKind,
        DiversityIndexKindParseError, SpeciesRichness,
    };
    pub use use_biome::{BiomeClimate, BiomeKind, BiomeKindParseError, BiomeName, BiomeTextError};
    pub use use_community::{
        CommunityComposition, CommunityKind, CommunityKindParseError, CommunityName, CommunityRole,
        CommunityRoleParseError, CommunityTextError,
    };
    pub use use_conservation_status::{
        ConservationStatus, ConservationStatusParseError, ConservationStatusSystem,
        ConservationTextError, ProtectionStatus, ThreatKind, ThreatKindParseError,
    };
    pub use use_ecosystem::{
        EcosystemComponent, EcosystemKind, EcosystemKindParseError, EcosystemName, EcosystemScale,
        EcosystemScaleParseError, EcosystemTextError,
    };
    pub use use_food_web::{
        EnergyFlowDirection, EnergyFlowDirectionParseError, FeedingRelation,
        FeedingRelationParseError, FoodWebLink, FoodWebName, FoodWebTextError,
    };
    pub use use_habitat::{
        HabitatCondition, HabitatConditionParseError, HabitatFeature, HabitatKind,
        HabitatKindParseError, HabitatName, HabitatTextError,
    };
    pub use use_niche::{
        NicheBreadth, NicheKind, NicheKindParseError, NicheName, NicheTextError, NicheValueError,
        ResourceUse,
    };
    pub use use_population::{
        PopulationDensity, PopulationId, PopulationName, PopulationSize, PopulationTextError,
        PopulationTrend, PopulationTrendParseError, PopulationValueError,
    };
    pub use use_species_interaction::{
        InteractionStrength, InteractionStrengthParseError, InteractionTextError,
        SpeciesInteraction, SpeciesInteractionKind, SpeciesInteractionKindParseError,
    };
    pub use use_trophic_level::{
        TrophicLevel, TrophicLevelParseError, TrophicPosition, TrophicRole, TrophicTextError,
        TrophicValueError,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        BiodiversityMeasure, BiomeKind, CommunityRole, ConservationStatus, EcosystemKind,
        FeedingRelation, FoodWebLink, HabitatKind, NicheKind, PopulationSize, SpeciesInteraction,
        SpeciesInteractionKind, SpeciesRichness, TrophicLevel,
    };

    #[test]
    fn facade_composes_ecology_primitives_without_simulation() {
        let ecosystem = EcosystemKind::Marine;
        let habitat = HabitatKind::Reef;
        let population = PopulationSize::new(150).expect("non-negative population");
        let role = CommunityRole::Keystone;
        let niche = NicheKind::Trophic;
        let biome = BiomeKind::Marine;
        let link = FoodWebLink::new("zooplankton", "anchovy", FeedingRelation::Consumes)
            .expect("valid food-web link");
        let interaction =
            SpeciesInteraction::new("coral", "algae", SpeciesInteractionKind::Mutualism)
                .expect("valid species interaction");
        let trophic = TrophicLevel::PrimaryConsumer;
        let richness = SpeciesRichness::new(128).expect("non-negative richness");
        let biodiversity = BiodiversityMeasure::SpeciesRichness(richness);
        let status = ConservationStatus::Vulnerable;

        assert_eq!(ecosystem.to_string(), "marine");
        assert_eq!(habitat.to_string(), "reef");
        assert_eq!(population.get(), 150);
        assert_eq!(role.to_string(), "keystone");
        assert_eq!(niche.to_string(), "trophic");
        assert_eq!(biome.to_string(), "marine");
        assert_eq!(link.to_string(), "zooplankton -[consumes]-> anchovy");
        assert_eq!(interaction.to_string(), "coral -[mutualism]-> algae");
        assert_eq!(trophic.to_string(), "primary-consumer");
        assert_eq!(biodiversity.to_string(), "species-richness: 128");
        assert_eq!(status.to_string(), "vulnerable");
    }
}
