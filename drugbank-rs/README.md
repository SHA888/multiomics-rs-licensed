# drugbank-rs

XML parser for DrugBank full-database export.

## License (Parser Code)

This crate is dual-licensed under MIT OR Apache-2.0.

## Data License (Important)

**This crate contains parser code only.** To parse real DrugBank data, you must obtain a separate license:

- **Academic**: Free license application + approval required at [go.drugbank.com](https://go.drugbank.com/releases/latest)
- **Commercial**: Paid license available from DrugBank

**Redistribution of parsed data is prohibited** without written permission from DrugBank.

## Usage

```rust
use drugbank_rs::DrugBankParser;

let parser = DrugBankParser::new();
// Parsing requires a DrugBank XML file obtained through proper licensing
```

## Status

Scaffold only — implementation pending.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
