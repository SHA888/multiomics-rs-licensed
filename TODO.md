# multiomics-rs-licensed — TODO

> Open source parser code (MIT OR Apache-2.0). Parsed data is governed by
> each upstream source's license terms — see [LICENSE-POLICY.md](LICENSE-POLICY.md).
> Each crate versions independently.
> Format: `[ ]` open · `[x]` done · `[-]` deferred

---

## Status (v0.1, 2026-04-20)

**Scaffold only.** No crates implemented. Workspace Cargo.toml, CI, and per-crate
scaffolds deferred until a concrete consumer need arises for a specific crate.
This TODO exists to record scope decisions, expected crate shape, and policy
boundaries before implementation begins.

## VERSIONING

```
v0.x.x   each crate: pre-stable, breaking changes allowed in minor
v1.0.0   per crate: stable public API, MSRV policy documented
```

Each crate publishes independently. No intra-workspace dependencies are expected
at v0.1 scope.

---

## SPRINT 0 — Workspace bootstrap  *(deferred)*
> Gate: first consumer request for any crate in this workspace.

- [ ] S0.1 Create `github.com/SHA888/multiomics-rs-licensed` (public)
- [ ] S0.2 `LICENSE-MIT` + `LICENSE-APACHE` (parser code)
- [ ] S0.3 `LICENSE-POLICY.md` (data license discipline — see workspace root)
- [ ] S0.4 `CONTRIBUTING.md` with license-discipline rules
- [ ] S0.5 `CODE_OF_CONDUCT.md` — Contributor Covenant v2.1
- [ ] S0.6 `SECURITY.md` — data correctness bugs = security severity
- [ ] S0.7 Workspace `Cargo.toml`, same template as `multiomics-rs`
- [ ] S0.8 `rust-toolchain.toml` pinning latest stable
- [ ] S0.9 `deny.toml` — same policy as `multiomics-rs`
- [ ] S0.10 CI workflow (ci.yml, release.yml, audit.yml)
- [ ] S0.11 Empty crate scaffolds (drugbank-rs, phosphositeplus-rs, oncokb-rs,
           disgenet-rs) with stub `src/lib.rs`, `README.md`, `CHANGELOG.md`
- [ ] S0.12 `cargo check --workspace` passes on empty scaffold
- [ ] S0.13 CI green on `main`

---

## drugbank-rs — v0.1.0  *(planned)*
> XML parser for DrugBank full-database export.

**Access requirement:** Academic free license (application + approval) or paid
commercial license. Signed agreement required. No data redistribution. See
`https://go.drugbank.com/releases/latest` for current terms.

- [ ] DB1.1 XML reader via `quick-xml` — streaming parse
  - `Drug` → RecordBatch (drugbank_id, name, groups, indication, mechanism,
    toxicity, metabolism, half_life, atc_codes (List<Utf8>))
  - `DrugTarget` → RecordBatch (drugbank_id, uniprot_id, organism, actions)
  - `DrugEnzyme` → RecordBatch (drugbank_id, uniprot_id, kind)
  - `DrugTransporter` → RecordBatch (drugbank_id, uniprot_id, direction)
- [ ] DB1.2 Synthetic XML fixtures only — no real DrugBank data in repo
- [ ] DB1.3 CI runs against synthetic fixtures only
- [ ] DB1.4 README prominently documents license requirement
- [ ] DB1.5 Arrow schema metadata carries source identifier + version + "academic or
          commercial license required — see DrugBank terms"
- [ ] DB1.6 Tests + publish `0.1.0`

---

## phosphositeplus-rs — v0.1.0  *(planned)*
> TSV parsers for PhosphoSitePlus post-translational modification tables.

**Access requirement:** Academic registration at `phosphosite.org`. No redistribution
of downloaded files. See PhosphoSitePlus terms of use.

- [ ] PP1.1 TSV reader for each PSP dataset (phosphorylation, acetylation,
           methylation, ubiquitination, sumoylation, O-GlcNAc, O-GalNAc)
  - `ModifiedProtein` → RecordBatch (uniprot_id, gene, residue, position,
    modification_type, organism, reference_count, domain, kinase)
- [ ] PP1.2 Kinase-substrate dataset reader separately
  - `KinaseSubstrate` → RecordBatch (kinase_uniprot, substrate_uniprot,
    site, in_vivo, in_vitro)
- [ ] PP1.3 Disease-associated sites reader
  - `DiseaseSite` → RecordBatch (uniprot_id, site, disease, pmid)
- [ ] PP1.4 Synthetic TSV fixtures only — no real PSP data in repo
- [ ] PP1.5 README documents registration requirement
- [ ] PP1.6 Arrow schema metadata carries license info
- [ ] PP1.7 Tests + publish `0.1.0`

---

## oncokb-rs — v0.1.0  *(planned)*
> Reader for OncoKB API output and/or licensed flat-file exports.

**Access requirement:** Academic or commercial license tiers. API access requires
an OncoKB token obtained via license approval. See `oncokb.org/terms`.

- [ ] OK1.1 API response reader (JSON → Arrow)
  - `ClinicallyRelevantVariant` → RecordBatch (gene, variant,
    oncogenic_effect, level_of_evidence, treatment, cancer_type, mutation_effect)
- [ ] OK1.2 Flat-file reader for licensed OncoKB data dumps
- [ ] OK1.3 Decision at implementation: API-only vs. flat-file vs. both —
           depends on what license tiers deliver
- [ ] OK1.4 Synthetic fixtures only
- [ ] OK1.5 README documents license tier options
- [ ] OK1.6 Arrow schema metadata carries license info + tier
- [ ] OK1.7 Tests + publish `0.1.0`

---

## disgenet-rs — v0.1.0  *(planned)*
> Reader for DISGENET data — Free Academic plan (curated, API-only) or
> paid Standard / Advanced plan (full database).

**Access requirement:** DISGENET Free Academic License requires institutional
email and manual application review (~7 business days per DISGENET support
docs). Free Academic plan provides access to curated data only with API result
limits. Full-database bulk download requires paid Standard or Advanced plan.
See `disgenet.com` for current terms.

- [ ] DG1.1 REST API client (Free Academic tier)
  - Authenticated requests; respects API result limits
  - `GeneDiseaseAssociation` → RecordBatch (gene_symbol, disease_name,
    disease_type, score, evidence_level, sources)
- [ ] DG1.2 TSV reader (paid Standard / Advanced tier — full-database downloads)
- [ ] DG1.3 Behavior documentation: crate works with either tier; feature-flag
           or runtime config distinguishes API-only from bulk-file paths
- [ ] DG1.4 Synthetic fixtures only
- [ ] DG1.5 README prominently documents academic application process and
           Free vs. paid plan differences
- [ ] DG1.6 Arrow schema metadata carries license tier info
- [ ] DG1.7 Tests + publish `0.1.0`

---

## FUTURE — Not currently planned

No post-v1.0 crates planned for this workspace. If another license-restricted
molecular reference database becomes a concrete consumer need, a new crate
can be added following the same discipline. Likely candidates if ever needed:

| Candidate | Source | Access |
|---|---|---|
| `snomed-ct-rs` | SNOMED CT (US edition) | UMLS Metathesaurus login + license |
| `medra-rs` | MedDRA terminology | ICH-MedDRA MSSO subscriber license |
| `atc-who-licensed-rs` | WHO ATC/DDD full | WHO Collaborating Centre purchase |

All three would be clinical-terminology, not strictly multi-omics — they
might belong in a fourth workspace, `clinical-terminology-licensed-rs`, rather
than here. Deferred entirely.

---

*Last updated: 2026-04-20. Scaffold-only. v0.1 records scope and policy before
any implementation begins. No active work; Sprint 0 deferred.*
