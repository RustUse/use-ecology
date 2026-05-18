# use-biome

Primitive biome vocabulary.

use-biome models non-empty biome names, descriptive biome kinds, and simple biome climate labels. It does not model climate zones, fetch biome maps, classify locations into biomes, or duplicate meteorology behavior.

## Example

`ust
use use_biome::{BiomeClimate, BiomeKind, BiomeName};

let name = BiomeName::new("temperate rainforest").unwrap();
let climate = BiomeClimate::new("cool and humid").unwrap();

assert_eq!(name.to_string(), "temperate rainforest");
assert_eq!(BiomeKind::TemperateForest.to_string(), "temperate-forest");
assert_eq!(climate.to_string(), "cool and humid");
`

## Scope

- biome names and labels
- descriptive biome kinds
- simple biome climate labels

## Non-goals

- climate zone modeling
- biome map fetching
- location classification into biomes
- meteorology duplication

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
