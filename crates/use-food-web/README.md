# use-food-web

Primitive food-web vocabulary.

use-food-web models non-empty food-web names, descriptive feeding relations and energy-flow directions, and simple food-web links between organism labels. It does not simulate energy transfer, calculate biomass flow, build graph algorithms beyond trivial relationship storage, or infer feeding relationships.

## Example

```rust
use use_food_web::{EnergyFlowDirection, FeedingRelation, FoodWebLink, FoodWebName};

let name = FoodWebName::new("reef shelf web").unwrap();
let link = FoodWebLink::new("zooplankton", "anchovy", FeedingRelation::Consumes).unwrap();

assert_eq!(name.to_string(), "reef shelf web");
assert_eq!(link.to_string(), "zooplankton -[consumes]-> anchovy");
assert_eq!(EnergyFlowDirection::PreyToPredator.to_string(), "prey-to-predator");
```

## Scope

- food-web names and labels
- descriptive feeding relations and energy-flow directions
- simple relationship links between organism labels

## Non-goals

- energy transfer simulation
- biomass flow calculations
- graph algorithm surfaces beyond trivial storage
- feeding relationship inference

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
