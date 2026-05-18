# use-trophic-level

Primitive trophic-level vocabulary.

use-trophic-level models descriptive trophic levels and roles plus non-negative trophic position values. It does not calculate trophic position from isotope data, model energy pyramids, or simulate food chains.

## Example

`ust
use use_trophic_level::{TrophicLevel, TrophicPosition, TrophicRole};

let role = TrophicRole::new("reef grazer").unwrap();
let position = TrophicPosition::new(2.0).unwrap();

assert_eq!(TrophicLevel::PrimaryConsumer.to_string(), "primary-consumer");
assert_eq!(role.to_string(), "reef grazer");
assert_eq!(position.get(), 2.0);
`

## Scope

- descriptive trophic levels and roles
- non-negative trophic position values

## Non-goals

- isotope-derived trophic position calculation
- energy pyramid modeling
- food-chain simulation

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
