//! Core Gentoo types and utilities
//!
//! This crate provides fundamental Gentoo-specific types that can be used
//! across various Gentoo-related Rust projects.

mod arch;
mod error;
mod variant;

pub use arch::Arch;
pub use error::Error;
pub use variant::Variant;
