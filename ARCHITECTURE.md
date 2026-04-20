# multiomics-rs-licensed Architecture

## Purpose

`multiomics-rs-licensed` provides Rust parsers that emit Apache Arrow RecordBatches
from biomedical reference databases requiring license agreements or registration
beyond a simple institutional email. Parser code is permissively licensed
(MIT OR Apache-2.0); the data these parsers read is governed by each upstream
source's terms and is the user's responsibility to obtain.

## Boundary with `multiomics-rs`

The test for whether a crate belongs in this workspace versus `multiomics-rs`:

| Requirement | → `multiomics-rs` | → `multiomics-rs-licensed` |
|---|---|---|
| Freely downloadable, no account | ✅ | — |
| Institutional email + clickthrough only | ✅ | — |
| Signed license agreement required | — | ✅ |
| Academic-use-only with no redistribution | — | ✅ |
| Full-database access requires paid tier | — | ✅ |
| Data redistribution prohibited in any form | — | ✅ |

CORUM, SIGNOR, HPA, SIDER, and CGI (CC BY-NC-SA) stay in `multiomics-rs` even though
their licenses restrict commercial use — the test is *access method*, not *license
restrictiveness*. A consumer wanting CC BY-NC data can download it, parse it, and
comply with terms downstream. A consumer wanting DrugBank data must first sign an
agreement with DrugBank; the `drugbank-rs` crate cannot provide the data, so the
boundary is meaningful.

## Arrow as the universal contract

Same as `multiomics-rs`. Every crate emits `arrow::record_batch::RecordBatch`.
No custom serialization, no graph output, no framework lock-in.

## Crate map

```
multiomics-rs-licensed workspace
│
├── drugbank-rs         Drug data → Arrow
│     Reads DrugBank full-database XML (user-obtained)
│     Emits: Drug { drugbank_id, name, groups, indication, mechanism,
│            toxicity, metabolism, half_life, targets, enzymes,
│            transporters, atc_codes }
│     License: academic free, commercial paid; no redistribution
│
├── phosphositeplus-rs  Post-translational modifications → Arrow
│     Reads PhosphoSitePlus TSV dumps (user-obtained)
│     Emits: ModifiedProtein { uniprot_id, residue, modification_type,
│            organism, reference_count, function_annotation }
│     License: academic free with registration; no redistribution
│
├── oncokb-rs           Clinically annotated cancer variants → Arrow
│     Reads OncoKB API output or flat files (user-obtained via licensed access)
│     Emits: ClinicallyRelevantVariant { gene, variant, oncogenic_effect,
│            level_of_evidence, treatment, cancer_type }
│     License: academic / commercial tiers
│
└── disgenet-rs         Gene-disease associations → Arrow
      Reads DISGENET curated subset (via API with Free Academic License)
      or full database (via paid Standard / Advanced plan)
      Emits: GeneDiseaseAssociation { gene_symbol, disease_name,
             disease_type, score, evidence_level, sources }
      License: Free Academic (curated only, registration-gated) /
               Standard / Advanced (paid, full database)
```

## License-discipline posture

Every crate in this workspace:

1. **Ships no data.** No test fixtures derived from real licensed data live in the repo.
   Synthetic fixtures only, structurally mirroring the format without containing
   real content.
2. **Ships no integration tests against real data.** CI runs against synthetic
   fixtures only. Users running tests with real data do so locally.
3. **Documents the access requirement prominently.** Each crate's README states
   what license / registration the user needs before the crate is useful.
4. **Surfaces license metadata in Arrow output.** Every emitted RecordBatch carries
   schema metadata identifying the source, version, and license so downstream
   consumers cannot lose track of the data's terms.
5. **Separates parser code from data.** The crate takes a file path or byte stream
   as input — it does not download, cache, or redistribute data. Users independently
   obtain access and provide the data to the parser.

## Dependency rules

```
Licensed crates      no dependency on each other
                     no dependency on clinical-rs
                     no dependency on multiomics-rs
                     no dependency on biomedref-rs
                     no dependency on any consuming application

Applications         depend on multiomics-rs-licensed crates (consumer)
                     multiomics-rs-licensed never imports application code
```

## Format notes

*Deferred to per-crate README.md files at implementation time. Each crate will
document its source file format, download procedure (from the user's licensed
access), and schema mapping. These format notes are intentionally not in this
ARCHITECTURE.md because (a) the details vary substantially per source and
(b) documenting them here prematurely before implementation invites drift.*

## Repository structure

```
multiomics-rs-licensed/
├── crates/
│   ├── drugbank-rs/          # planned, v0.1.0
│   ├── phosphositeplus-rs/   # planned, v0.1.0
│   ├── oncokb-rs/            # planned, v0.1.0
│   └── disgenet-rs/          # planned, v0.1.0
├── ARCHITECTURE.md
├── TODO.md
├── README.md
├── LICENSE-POLICY.md         # workspace license-discipline rules
├── CONTRIBUTING.md
├── LICENSE-MIT
├── LICENSE-APACHE
└── Cargo.toml                # planned, not yet created
```

## Status

`v0.1` — scaffold documentation only. Workspace bootstrap deferred until a
concrete consumer need for a specific crate emerges.
