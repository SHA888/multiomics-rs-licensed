<div align="center">

# multiomics-rs-licensed

**Rust Arrow parsers for molecular reference databases requiring academic or commercial license agreements.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)

[Architecture](ARCHITECTURE.md) · [Roadmap](TODO.md) · [License Policy](LICENSE-POLICY.md)

</div>

---

## What is this?

`multiomics-rs-licensed` is a Cargo workspace of Rust parsers for biomedical reference
databases that require signed license agreements, academic-use terms that prohibit
redistribution, or paid commercial licenses for full-database access.

**This workspace contains parser code only. It does not contain data, test fixtures, or
examples derived from licensed sources. Obtaining the data is the user's responsibility
and is governed by each upstream source's license terms.**

| Crate | Source | Access requirement | Status |
|---|---|---|---|
| [`drugbank-rs`](crates/drugbank-rs) | DrugBank | Academic / commercial license, signed agreement | 📋 Planned |
| [`phosphositeplus-rs`](crates/phosphositeplus-rs) | PhosphoSitePlus | Academic registration, no redistribution | 📋 Planned |
| [`oncokb-rs`](crates/oncokb-rs) | OncoKB | Academic / commercial license tiers | 📋 Planned |
| [`disgenet-rs`](crates/disgenet-rs) | DISGENET | Free Academic License (institutional email, application review), full DB requires paid Standard / Advanced plan | 📋 Planned |

All four crates are **planned, not yet implemented** as of v0.1 of this document
(2026-04-20). The workspace currently exists at the scaffold level (README,
ARCHITECTURE, TODO) pending a concrete consumer need for any specific crate.

## Why a separate workspace?

A single permissive workspace (`multiomics-rs`) with license-warnings scattered through
per-crate READMEs was considered and rejected. The problems:

1. **Contributors and auditors need to know the boundary by looking.** A separate
   workspace makes "these crates require extra access steps" visually obvious at the
   repository level, not buried in each crate's README.
2. **CI cannot test licensed-data parsers against real data.** A workspace combining
   open and licensed sources either has mixed CI posture (some crates have integration
   tests with real data, others don't) or has to degrade all to synthetic fixtures.
3. **Legal posture is cleaner.** A workspace whose sole purpose is "parsers for
   licensed sources" lets contributors and users think about the legal obligations
   explicitly rather than stumbling into them.

See [LICENSE-POLICY.md](LICENSE-POLICY.md) for the workspace's license-discipline rules.

## Sibling workspaces

```
clinical-rs              clinical records (MIMIC, ICD codes, task windowing)
                         github.com/SHA888/clinical-rs

multiomics-rs            molecular reference databases, fully open, no registration
                         github.com/SHA888/multiomics-rs

multiomics-rs-licensed   this workspace — molecular references requiring license
                         agreements or registration
                         github.com/SHA888/multiomics-rs-licensed

biomedref-rs             biomedical references outside strict molecular omics
                         (literature-mining, environmental exposure, food composition)
                         github.com/SHA888/biomedref-rs
```

None of the four workspaces depends on any other. All four emit Apache Arrow
RecordBatches as the common contract. Consumers declare dependencies on whichever
workspaces they need.

## Quick start — expected usage pattern

When crates in this workspace ship, the typical consumer workflow will be:

```toml
# Cargo.toml
[dependencies]
drugbank-rs = "0.1"   # when published
```

```rust
use drugbank_rs::DrugBankReader;

// User independently obtained DrugBank data files per their license agreement
let reader = DrugBankReader::from_xml("path/to/drugbank_all_full_database.xml")?;
let batches = reader.drug_batches()?;
```

The crate provides parsing; the user provides data files.

## Status

`v0.1` — Scaffold-level documentation only. No crates implemented. Workspace bootstrap
(Cargo.toml, CI, crate stubs) deferred until concrete consumer need for a specific crate.

## License

Parser code dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE),
at your option. **The MIT / Apache-2.0 license applies to this workspace's parser code
only. Data accessed via these parsers remains under the upstream source's original
license terms.** See [LICENSE-POLICY.md](LICENSE-POLICY.md).

## Citation

```bibtex
@software{multiomics_rs_licensed,
  author  = {Kresna Sucandra},
  title   = {multiomics-rs-licensed: Rust Arrow parsers for license-restricted biomedical reference databases},
  url     = {https://github.com/SHA888/multiomics-rs-licensed},
  license = {MIT OR Apache-2.0},
}
```
