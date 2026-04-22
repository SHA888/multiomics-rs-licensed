//! OncoKB API and data parser
//!
//! This crate provides parsing for OncoKB API responses and licensed flat-file
//! exports.
//!
//! # Data License Notice
//!
//! **This crate contains parser code only.** To use this parser with real data,
//! you must:
//!
//! 1. Obtain an OncoKB license (academic or commercial tier)
//! 2. Get an API token or flat-file export through proper licensing
//! 3. Comply with OncoKB terms at [oncokb.org/terms](https://www.oncokb.org/terms)
//!
//! # Example
//!
//! ```rust
//! use oncokb_rs;
//!
//! // Parsing requires OncoKB API access or licensed data export
//! // This crate provides the parser — you must supply the licensed access
//! ```

#![doc = include_str!("../README.md")]

/// Placeholder for future OncoKB parser implementation.
pub struct OncoKbParser;

impl OncoKbParser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for OncoKbParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = OncoKbParser::new();
        assert_eq!(
            std::mem::size_of_val(&parser),
            0,
            "Parser is ZST as expected"
        );
    }
}
