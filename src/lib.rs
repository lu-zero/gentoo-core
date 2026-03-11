//! Core Gentoo types and utilities

mod arch;
mod error;
mod interner;
mod variant;

pub use arch::{Arch, ExoticKey, KnownArch};
pub use error::Error;
#[cfg(feature = "interner")]
pub use interner::GlobalInterner;
pub use interner::{DefaultInterner, Interned, Interner, NoInterner};
pub use variant::Variant;
