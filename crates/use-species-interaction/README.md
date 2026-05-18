# use-species-interaction

Primitive species interaction vocabulary.

use-species-interaction models descriptive interaction kinds and strengths plus simple pairwise interactions between species or organism labels. It does not model population outcomes, simulate interactions, calculate interaction networks, or fetch interaction data.

## Example

`ust
use use_species_interaction::{InteractionStrength, SpeciesInteraction, SpeciesInteractionKind};

let interaction = SpeciesInteraction::new("coral", "algae", SpeciesInteractionKind::Mutualism)
    .unwrap()
    .with_strength(InteractionStrength::Strong);

assert_eq!(interaction.to_string(), "coral -[mutualism]-> algae");
assert_eq!(interaction.strength(), Some(&InteractionStrength::Strong));
`

## Scope

- descriptive species interaction kinds and strengths
- simple pairwise interaction labels

## Non-goals

- population outcome modeling
- interaction simulation
- interaction network calculation
- interaction data fetching

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
