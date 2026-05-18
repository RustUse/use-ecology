# use-ecology

RustUse is “Composable sets of primitive Rust utility crates for fellow crustaceans.”

`use-ecology` is a primitive ecology vocabulary set. It provides small, composable Rust primitives for describing ecosystems, habitats, populations, communities, niches, biomes, food webs, species interactions, trophic levels, biodiversity measures, and conservation status labels.

`use-ecology` is not an ecology simulator, population model, biodiversity database, GIS engine, conservation-planning tool, food-web simulator, species-distribution model, climate model, ecosystem services calculator, or environmental science framework. It does not simulate ecosystems, predict outcomes, fetch external data, map habitats, render geography, or perform conservation planning.

## Boundaries

- `use-biology` owns organisms, taxonomy, species, cells, genes, traits, tissues, organs, biological systems, life stages, and reproduction.
- `use-geography` owns coordinates, places, geographic regions, boundaries, projections, and spatial references.
- `use-meteorology` owns weather and atmosphere primitives.
- `use-geology` owns minerals, rocks, strata, faults, plates, and geologic time.

`use-ecology` complements those sets by focusing on ecological relationships and environment-level vocabulary instead of biological identity, GIS behavior, atmospheric modeling, or geologic classification.

## Crates

- `use-ecology`: facade crate for the full primitive ecology vocabulary set
- `use-ecosystem`: primitive ecosystem vocabulary
- `use-habitat`: primitive habitat vocabulary
- `use-population`: primitive population vocabulary
- `use-community`: primitive ecological community vocabulary
- `use-niche`: primitive niche vocabulary
- `use-biome`: primitive biome vocabulary
- `use-food-web`: primitive food-web relationship vocabulary
- `use-species-interaction`: primitive species interaction vocabulary
- `use-trophic-level`: primitive trophic-level vocabulary
- `use-biodiversity`: primitive biodiversity measure vocabulary
- `use-conservation-status`: primitive conservation status vocabulary

## Scope

- small focused crates
- primitives over frameworks
- few or no dependencies
- stable APIs
- Rust 2024 edition
- strong documentation
- meaningful tests
- composable exports
- dual MIT OR Apache-2.0 license
- no unnecessary macros
- no async
- no global runtime assumptions
- no unsafe code
- no network calls
- no database fetching
- no simulation
- no prediction engine
- no GIS behavior

## Example

```rust
use use_ecology::prelude::{
    BiodiversityMeasure, BiomeKind, CommunityRole, ConservationStatus, EcosystemKind,
    FeedingRelation, FoodWebLink, HabitatKind, NicheKind, PopulationSize, SpeciesInteraction,
    SpeciesInteractionKind, SpeciesRichness, TrophicLevel,
};

let ecosystem = EcosystemKind::Wetland;
let habitat = HabitatKind::Reef;
let population = PopulationSize::new(420).unwrap();
let community_role = CommunityRole::Keystone;
let niche = NicheKind::Realized;
let biome = BiomeKind::Marine;
let food_web = FoodWebLink::new("krill", "penguin", FeedingRelation::Consumes).unwrap();
let interaction = SpeciesInteraction::new("coral", "algae", SpeciesInteractionKind::Mutualism)
    .unwrap();
let trophic_level = TrophicLevel::PrimaryConsumer;
let biodiversity = BiodiversityMeasure::SpeciesRichness(SpeciesRichness::new(128).unwrap());
let conservation = ConservationStatus::Vulnerable;

assert_eq!(ecosystem.to_string(), "wetland");
assert_eq!(habitat.to_string(), "reef");
assert_eq!(population.get(), 420);
assert_eq!(community_role.to_string(), "keystone");
assert_eq!(niche.to_string(), "realized");
assert_eq!(biome.to_string(), "marine");
assert_eq!(food_web.to_string(), "krill -[consumes]-> penguin");
assert_eq!(interaction.to_string(), "coral -[mutualism]-> algae");
assert_eq!(trophic_level.to_string(), "primary-consumer");
assert_eq!(biodiversity.to_string(), "species-richness: 128");
assert_eq!(conservation.to_string(), "vulnerable");
```

This example describes ecology concepts. It does not simulate populations, fetch species data, map habitats, predict species distributions, or perform conservation planning.

## Related Sets

- `use-biology`
- `use-geography`
- `use-geology`
- `use-meteorology`
- `use-data`
- `use-validate`
- `use-measure`
- `use-units`

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
