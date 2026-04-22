# phosphositeplus-rs

TSV parsers for PhosphoSitePlus post-translational modification tables.

## License (Parser Code)

This crate is dual-licensed under MIT OR Apache-2.0.

## Data Access (Important)

**This crate contains parser code only.** To parse real PhosphoSitePlus data, you must:

1. **Register** for academic access at [phosphosite.org](https://www.phosphosite.org)
2. **Download** the TSV files through your registered account
3. **Comply** with PhosphoSitePlus terms of use (no redistribution)

## Usage

```rust
use phosphositeplus_rs::PhosphoSitePlusParser;

let parser = PhosphoSitePlusParser::new();
// Parsing requires PhosphoSitePlus TSV files obtained through registration
```

## Supported Datasets

Planned support for:
- Phosphorylation sites
- Acetylation sites
- Methylation sites
- Ubiquitination sites
- Sumoylation sites
- O-GlcNAc sites
- O-GalNAc sites
- Kinase-substrate relationships
- Disease-associated sites

## Status

Scaffold only — implementation pending.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
