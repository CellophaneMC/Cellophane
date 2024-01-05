//! Basic components for cellophanemc server development.
//!
//! # Features
//!
#![doc = document_features::document_features!()]
//!

// This crate combines higher-level abstractions from other crates so `unsafe` is a red flag here
#![deny(unsafe_code)]

#[cfg(feature = "core")]
pub use cellophanemc_core as core;
pub use cellophanemc_network as network;
pub use cellophanemc_protocol as protocol;
