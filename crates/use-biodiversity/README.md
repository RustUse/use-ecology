# use-biodiversity

Primitive biodiversity vocabulary.

use-biodiversity models non-negative species richness values, descriptive diversity index kinds, and biodiversity measures that wrap species richness or named index values. It does not implement biodiversity analysis frameworks, fetch occurrence data, calculate indices from datasets, or perform conservation scoring.

## Example

```rust
use use_biodiversity::{BiodiversityMeasure, DiversityIndex, DiversityIndexKind, SpeciesRichness};

let richness = SpeciesRichness::new(128).unwrap();
let shannon = DiversityIndex::new(DiversityIndexKind::Shannon, 2.3).unwrap();

assert_eq!(BiodiversityMeasure::SpeciesRichness(richness).to_string(), "species-richness: 128");
assert_eq!(BiodiversityMeasure::DiversityIndex(shannon).to_string(), "shannon: 2.3");
```

## Scope

- non-negative species richness values
- descriptive diversity index kinds
- biodiversity measures that carry named values

## Non-goals

- biodiversity analysis frameworks
- occurrence data fetching
- index calculation from datasets
- conservation scoring

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
