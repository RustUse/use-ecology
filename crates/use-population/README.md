# use-population

Primitive ecological population vocabulary.

use-population models non-empty population identifiers and names, non-negative population sizes and densities, and descriptive population trends. It does not model population dynamics, simulate growth, estimate population size, or implement demographic models.

## Example

```rust
use use_population::{PopulationDensity, PopulationId, PopulationName, PopulationSize, PopulationTrend};

let id = PopulationId::new("marsh-herons")?;
let name = PopulationName::new("Marsh herons")?;
let size = PopulationSize::new(420)?;
let density = PopulationDensity::new(3.5)?;

assert_eq!(id.to_string(), "marsh-herons");
assert_eq!(name.to_string(), "Marsh herons");
assert_eq!(size.get(), 420);
assert_eq!(density.get(), 3.5);
assert_eq!(PopulationTrend::Stable.to_string(), "stable");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- population identifiers and names
- non-negative population sizes and densities
- descriptive population trend labels

## Non-goals

- population dynamics modeling
- growth simulation
- population estimation
- demographic models

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
