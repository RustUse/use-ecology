# use-ecosystem

Primitive ecosystem vocabulary.

`use-ecosystem` models non-empty ecosystem names, descriptive ecosystem kinds and scales, and simple ecosystem component labels. It does not simulate ecosystems, calculate ecosystem services, fetch data, or model energy flow.

## Example

```rust
use use_ecosystem::{EcosystemComponent, EcosystemKind, EcosystemName, EcosystemScale};

let name = EcosystemName::new("coastal marsh").unwrap();
let component = EcosystemComponent::new("salt-tolerant grasses").unwrap();

assert_eq!(name.to_string(), "coastal marsh");
assert_eq!(EcosystemKind::Wetland.to_string(), "wetland");
assert_eq!("regional".parse::<EcosystemScale>().unwrap(), EcosystemScale::Regional);
assert_eq!(component.to_string(), "salt-tolerant grasses");
```

## Scope

- ecosystem names and labels
- descriptive ecosystem kinds and scales
- descriptive ecosystem component labels

## Non-goals

- ecosystem simulation
- ecosystem services calculations
- external ecosystem data fetching
- energy flow modeling

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
