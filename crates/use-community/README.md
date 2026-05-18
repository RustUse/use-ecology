# use-community

Primitive ecological community vocabulary.

use-community models non-empty community names, descriptive community kinds and roles, and deterministic community composition labels. It does not infer community structure, compute diversity indices, model succession, or fetch species data.

## Example

`ust
use use_community::{CommunityComposition, CommunityKind, CommunityName, CommunityRole};

let name = CommunityName::new("reef fish assemblage")?;
let composition = CommunityComposition::new(["parrotfish", "grouper", "wrasse"])?;

assert_eq!(name.to_string(), "reef fish assemblage");
assert_eq!(CommunityKind::AquaticCommunity.to_string(), "aquatic-community");
assert_eq!(CommunityRole::Keystone.to_string(), "keystone");
assert_eq!(composition.iter().next(), Some(&"grouper".to_string()));
# Ok::<(), use_community::CommunityTextError>(())
`

## Scope

- community names and labels
- descriptive community kinds and roles
- deterministic community composition labels

## Non-goals

- community structure inference
- diversity index calculations
- succession modeling
- species data fetching

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
