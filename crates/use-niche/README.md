# use-niche

Primitive ecological niche vocabulary.

use-niche models non-empty niche names, descriptive niche kinds, resource-use labels, and non-negative niche breadth values. It does not infer niche overlap, model competition, implement species-distribution modeling, or calculate niche breadth from data.

## Example

```rust
use use_niche::{NicheBreadth, NicheKind, NicheName, ResourceUse};

let name = NicheName::new("reef grazer").unwrap();
let resource = ResourceUse::new("filamentous algae").unwrap();
let breadth = NicheBreadth::new(2.5).unwrap();

assert_eq!(name.to_string(), "reef grazer");
assert_eq!(NicheKind::Trophic.to_string(), "trophic");
assert_eq!(resource.to_string(), "filamentous algae");
assert_eq!(breadth.get(), 2.5);
```

## Scope

- niche names and labels
- descriptive niche kinds
- resource-use labels
- non-negative niche breadth values

## Non-goals

- niche overlap inference
- competition modeling
- species-distribution modeling
- niche breadth calculation from data

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
