# use-ecology

Facade crate for primitive `RustUse` ecology vocabulary.

`use-ecology` re-exports focused crates for ecosystems, habitats, populations, communities, niches, biomes, food webs, species interactions, trophic levels, biodiversity, and conservation status. It is not an ecology simulator, population model, climate model, biodiversity database, conservation-planning system, GIS engine, food-web simulator, species-distribution model, or environmental science framework.

```rust
use use_ecology::prelude::{
    BiodiversityMeasure, BiomeKind, CommunityRole, ConservationStatus, EcosystemKind,
    FeedingRelation, FoodWebLink, HabitatKind, NicheKind, PopulationSize,
    SpeciesInteraction, SpeciesInteractionKind, SpeciesRichness, TrophicLevel,
};

let ecosystem = EcosystemKind::Marine;
let habitat = HabitatKind::Reef;
let population = PopulationSize::new(150).unwrap();
let role = CommunityRole::Keystone;
let niche = NicheKind::Trophic;
let biome = BiomeKind::Marine;
let link = FoodWebLink::new("zooplankton", "anchovy", FeedingRelation::Consumes).unwrap();
let interaction =
    SpeciesInteraction::new("coral", "algae", SpeciesInteractionKind::Mutualism).unwrap();
let trophic = TrophicLevel::PrimaryConsumer;
let richness = SpeciesRichness::new(128).unwrap();
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
```

This crate describes ecology concepts. It does not simulate ecosystems, analyze observations, infer conservation outcomes, or fetch environmental data.
