# use-habitat

Primitive habitat vocabulary.

use-habitat models non-empty habitat names, descriptive habitat kinds and conditions, and simple habitat feature labels. It does not perform habitat suitability modeling, map habitat ranges, fetch environmental data, or implement conservation planning.

## Example

```rust
use use_habitat::{HabitatCondition, HabitatFeature, HabitatKind, HabitatName};

let name = HabitatName::new("riparian corridor").unwrap();
let feature = HabitatFeature::new("coarse woody debris").unwrap();

assert_eq!(name.to_string(), "riparian corridor");
assert_eq!(HabitatKind::River.to_string(), "river");
assert_eq!(HabitatCondition::Protected.to_string(), "protected");
assert_eq!(feature.to_string(), "coarse woody debris");
```

## Scope

- habitat names and labels
- descriptive habitat kinds and conditions
- descriptive habitat feature labels

## Non-goals

- habitat suitability modeling
- habitat range mapping
- environmental data fetching
- conservation planning

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
