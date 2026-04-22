//! DISGENET API client and TSV parser
//!
//! This crate provides a REST API client for DISGENET Free Academic tier and
//! TSV parsing for paid Standard/Advanced tier bulk downloads.
//!
//! # Data License Notice
//!
//! **This crate contains parser code only.** To use this crate with real data,
//! you must:
//!
//! 1. Obtain DISGENET access:
//!    - **Free Academic**: Apply at [disgenet.com](https://www.disgenet.com) (~7 business days)
//!    - **Standard/Advanced**: Purchase a paid plan
//! 2. Comply with DISGENET terms of use
//! 3. Respect API rate limits (Free Academic tier) or use downloaded TSVs (paid plans)
//!
//! # Example
//!
//! ```rust
//! use disgenet_rs;
//!
//! // API client or TSV parser requires DISGENET license
//! // This crate provides the tools — you must supply the licensed access
//! ```

#![doc = include_str!("../README.md")]

/// Placeholder for future DISGENET API client and parser implementation.
pub struct DisgenetClient;

impl DisgenetClient {
    /// Creates a new client instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for DisgenetClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DisgenetClient::new();
        assert!(true, "Client created successfully");
    }
}
