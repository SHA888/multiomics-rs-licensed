# Contributing to multiomics-rs-licensed

Thank you for your interest in contributing! This project follows specific license discipline rules that all contributors must understand and follow.

## License Discipline

### Code License

All Rust source code contributions are dual-licensed under **MIT OR Apache-2.0**. By submitting a pull request, you agree that your contribution may be used under either license.

### Data License Discipline (Critical)

This workspace contains **parsers only** — no real data from licensed sources is ever committed to the repository.

#### You MUST NOT contribute:

- **Real data files** from DrugBank, PhosphoSitePlus, OncoKB, DISGENET, or any other licensed source (even "small samples" or "test extracts")
- **API keys, access tokens, or credentials** for any data source
- **Code that bypasses or circumvents** access controls, rate limits, or authentication requirements
- **Documentation that encourages** violation of upstream license terms

#### You MAY contribute:

- **Synthetic test fixtures** that mimic the format but contain no real data
- **Parser code** that reads publicly documented formats
- **Documentation** of access requirements and proper usage

### Pre-Submission Checklist

Before submitting a PR, verify:

- [ ] No real licensed data is included in the commit
- [ ] All test fixtures are synthetic/generated
- [ ] No credentials or API keys are hardcoded
- [ ] Arrow schema metadata includes appropriate license warnings
- [ ] README clearly documents upstream access requirements
- [ ] Code respects rate limits and API best practices

## Development Process

### Getting Started

1. Fork the repository
2. Ensure you have Rust stable toolchain installed
3. Run `cargo check --workspace` to verify the build
4. Run `cargo test --workspace` to run tests

### Testing

- All crates use **synthetic fixtures only**
- Tests run against generated data that mimics the format
- CI never accesses real data sources

### Code Style

- Follow standard Rust conventions (`cargo fmt`, `cargo clippy`)
- Document public APIs with rustdoc
- Include license metadata in Arrow schema outputs

### Pull Request Process

1. **Describe your changes** clearly in the PR description
2. **Reference any related issues**
3. **Confirm license compliance** — the PR template will ask you to confirm no real data is included
4. **Wait for CI** to pass (runs against synthetic fixtures only)
5. **Address review feedback**

## Questions?

- **Technical questions:** Open an issue with the `question` label
- **License concerns:** Open a discussion issue (do not include data samples)

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

*Thank you for contributing responsibly!*
