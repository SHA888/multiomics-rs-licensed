# oncokb-rs

Reader for OncoKB API output and/or licensed flat-file exports.

## License (Parser Code)

This crate is dual-licensed under MIT OR Apache-2.0.

## Data License (Important)

**This crate contains parser code only.** To access OncoKB data, you must obtain a separate license:

- **Academic License**: Free for academic research with institutional email
- **Commercial License**: Paid tier for commercial use

See [oncokb.org/terms](https://www.oncokb.org/terms) for current licensing options.

## Usage

```rust
use oncokb_rs::OncoKbParser;

let parser = OncoKbParser::new();
// Parsing requires OncoKB API token or licensed flat-file export
```

## Features (Planned)

- API response reader (JSON → Arrow)
- Flat-file reader for licensed data dumps
- Clinically relevant variant parsing
- Level of evidence extraction

## Status

Scaffold only — implementation pending.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
