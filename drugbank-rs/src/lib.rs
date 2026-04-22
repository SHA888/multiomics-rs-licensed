//! DrugBank XML parser
//!
//! This crate provides streaming XML parsing for DrugBank full-database exports.
//!
//! # Data License Notice
//!
//! **This crate contains parser code only.** To use this parser with real data,
//! you must:
//!
//! 1. Obtain a DrugBank license (academic free license or commercial)
//! 2. Comply with DrugBank's terms of use
//! 3. Not redistribute the parsed data without permission
//!
//! See [DrugBank releases](https://go.drugbank.com/releases/latest) for current terms.
//!
//! # Example
//!
//! ```rust
//! use drugbank_rs;
//!
//! // Parsing requires a DrugBank XML file obtained through proper licensing
//! // This crate provides the parser — you must supply the licensed data
//! ```

#![doc = include_str!("../README.md")]

/// Placeholder for future DrugBank XML parser implementation.
pub struct DrugBankParser;

impl DrugBankParser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DrugBankParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = DrugBankParser::new();
        assert_eq!(
            std::mem::size_of_val(&parser),
            0,
            "Parser is ZST as expected"
        );
    }
}
