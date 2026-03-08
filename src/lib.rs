//! Core Gentoo types and utilities

mod arch;
mod error;
mod interner;
mod variant;

pub use arch::{Arch, ExoticKey, KnownArch};
pub use error::Error;
pub use interner::{ArchInterner, GlobalArchInterner};
pub use variant::Variant;
