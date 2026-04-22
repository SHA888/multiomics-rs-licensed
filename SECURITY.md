# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please report it responsibly.

### How to Report

**Please do not open a public issue for security vulnerabilities.**

Instead, report privately via:
- GitHub Security Advisories: [Report a vulnerability](https://github.com/SHA888/multiomics-rs-licensed/security/advisories/new)
- Or email the maintainers directly (see repository owner profile)

### What to Include

- Type of vulnerability
- Steps to reproduce (if applicable)
- Affected versions
- Potential impact
- Suggested fix (if you have one)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 5 business days
- **Fix timeline**: Depends on severity (see below)
- **Disclosure**: Coordinated with reporter after fix is available

## Severity Classification

### Data Correctness Bugs = Security Severity

In multiomics data parsers, **incorrect data parsing is a security issue** because downstream scientific and clinical decisions depend on data accuracy.

| Severity | Examples | Response Time |
|----------|----------|---------------|
| **Critical** | Silent data corruption, incorrect field mapping leading to wrong scientific conclusions | 24-48 hours |
| **High** | Panics on valid input, unhandled edge cases in data format | 1 week |
| **Medium** | Performance degradation on malformed input, non-critical parsing errors | 2 weeks |
| **Low** | Documentation issues, test gaps | Next release |

### Examples of Security-Relevant Issues

- **Silent truncation**: Parser drops records without warning
- **Type confusion**: Numeric IDs parsed as strings or vice versa
- **Field misalignment**: Values mapped to wrong columns in Arrow output
- **Encoding errors**: Character encoding mishandling leading to data loss
- **Missing validation**: Invalid data accepted without error

## Security Best Practices for Users

1. **Validate outputs**: Always sanity-check parsed data against source expectations
2. **Use schema metadata**: Check Arrow schema for license and provenance information
3. **Monitor for updates**: Keep parsers updated for bug fixes
4. **Report anomalies**: If you observe unexpected parsing behavior, report it

## Disclosure Policy

We follow a coordinated disclosure process:

1. Reporter submits vulnerability privately
2. We investigate and confirm the issue
3. We develop and test a fix
4. We notify users with security advisories for critical/high issues
5. We publish the fix and publicly disclose (crediting reporter if desired)

---

*This security policy was last updated: 2026-04-22*
