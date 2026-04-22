# License Policy

> This document governs the relationship between **parser code licenses** and **data licenses** for all crates in this workspace.

## Summary

| Component | License | Notes |
|-----------|---------|-------|
| Parser code (Rust source) | MIT OR Apache-2.0 | Your choice of either license |
| Parsed data output | Varies by source | See per-crate documentation |
| Test fixtures (synthetic) | MIT OR Apache-2.0 | Generated, no real data |

## Parser Code License

All Rust source code in this repository is dual-licensed under:

- **MIT License** (see `LICENSE-MIT`)
- **Apache License 2.0** (see `LICENSE-APACHE`)

You may use, modify, and distribute the parser code under either license at your option.

## Data License Discipline

Each crate in this workspace parses data from a specific upstream source with its own license terms. **The parser code license does not apply to the data being parsed.**

### Core Principle

> **Users must independently obtain access to and comply with the license terms of each data source.**

### Per-Crate Data Sources

| Crate | Data Source | Access Requirement | Redistribution |
|-------|-------------|-------------------|----------------|
| `drugbank-rs` | DrugBank | Academic license application or commercial license | Prohibited |
| `phosphositeplus-rs` | PhosphoSitePlus | Academic registration | Prohibited |
| `oncokb-rs` | OncoKB | Academic or commercial license tier | Per license terms |
| `disgenet-rs` | DISGENET | Free Academic application or paid plan | Per license terms |

### User Obligations

Before using any crate to parse real data:

1. **Obtain legitimate access** to the data source through proper channels
2. **Review and accept** the upstream source's terms of use
3. **Respect redistribution restrictions** — parsed data may inherit the source's restrictions
4. **Do not commit real data** to this repository or any public fork

### Arrow Schema Metadata

All crates embed license metadata in Arrow schema output:

```rust
// Example metadata included in RecordBatch schema
"source": "DrugBank",
"source_version": "5.1.11",
"license_required": "Academic or commercial license — see DrugBank terms",
"redistribution": "Prohibited without written permission"
```

This metadata travels with the data to remind downstream users of their obligations.

## Contributing

By contributing to this repository, you agree that your contributions are dual-licensed MIT OR Apache-2.0.

**Critical:** Do not contribute:
- Real data from licensed sources (even "small samples")
- API keys or access credentials
- Code that bypasses access controls

## Synthetic Fixtures

All test fixtures are **synthetically generated** and contain no real data from licensed sources. They are MIT OR Apache-2.0 licensed.

## Questions

If you are uncertain about data licensing implications for your use case:

1. Consult the upstream source's terms directly
2. Contact your institution's legal/compliance office
3. Open a discussion issue (without sharing data samples)

---

*Last updated: 2026-04-22*
