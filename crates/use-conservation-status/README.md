# use-conservation-status

Primitive conservation status vocabulary.

use-conservation-status models descriptive conservation status labels, threat kinds, named status systems, and protection-status labels. It does not fetch conservation data, validate against external lists, provide legal advice, or implement conservation planning.

## Example

```rust
use use_conservation_status::{
    ConservationStatus, ConservationStatusSystem, ProtectionStatus, ThreatKind,
};

let system = ConservationStatusSystem::new("IUCN").unwrap();
let protection = ProtectionStatus::new("protected-area").unwrap();

assert_eq!(ConservationStatus::Vulnerable.to_string(), "vulnerable");
assert_eq!(ThreatKind::HabitatLoss.to_string(), "habitat-loss");
assert_eq!(system.to_string(), "IUCN");
assert_eq!(protection.to_string(), "protected-area");
```

## Scope

- descriptive conservation status labels
- descriptive threat kind labels
- named conservation status systems and protection labels

## Non-goals

- conservation data fetching
- validation against external lists
- legal advice
- conservation planning

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
