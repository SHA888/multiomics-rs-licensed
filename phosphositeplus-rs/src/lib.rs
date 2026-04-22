//! PhosphoSitePlus TSV parser
//!
//! This crate provides TSV parsing for PhosphoSitePlus post-translational
//! modification tables.
//!
//! # Data License Notice
//!
//! **This crate contains parser code only.** To use this parser with real data,
//! you must:
//!
//! 1. Register for academic access at [phosphosite.org](https://www.phosphosite.org)
//! 2. Comply with PhosphoSitePlus terms of use
//! 3. Not redistribute the downloaded data files
//!
//! # Example
//!
//! ```rust
//! use phosphositeplus_rs;
//!
//! // Parsing requires PhosphoSitePlus TSV files obtained through registration
//! // This crate provides the parser — you must supply the registered data
//! ```

#![doc = include_str!("../README.md")]

/// Placeholder for future PhosphoSitePlus TSV parser implementation.
pub struct PhosphoSitePlusParser;

impl PhosphoSitePlusParser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for PhosphoSitePlusParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = PhosphoSitePlusParser::new();
        assert_eq!(
            std::mem::size_of_val(&parser),
            0,
            "Parser is ZST as expected"
        );
    }
}
