# disgenet-rs

Reader for DISGENET data — Free Academic plan (curated, API-only) or paid Standard/Advanced plan (full database).

## License (Parser Code)

This crate is dual-licensed under MIT OR Apache-2.0.

## Data License (Important)

**This crate contains parser code only.** To access DISGENET data, you must obtain a separate license:

### Free Academic Plan
- Requires institutional email and manual application review (~7 business days)
- Provides curated data only via API
- Subject to API result limits

### Standard/Advanced Plan
- Paid plans for bulk database download
- Full access to all DISGENET data

See [disgenet.com](https://www.disgenet.com) for current terms.

## Usage

```rust
use disgenet_rs::DisgenetClient;

let client = DisgenetClient::new();
// API client or TSV parser requires DISGENET license
```

## Features

- `api` — REST API client for Free Academic tier
- `bulk` — TSV reader for paid tier bulk downloads

## Status

Scaffold only — implementation pending.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
