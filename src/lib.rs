//! Core Gentoo types and utilities

pub mod arch;
mod error;
pub mod interner;
pub mod variant;

pub use error::Error;

pub use arch::KnownArch;

/// A Gentoo architecture, either well-known or overlay-defined.
pub type Arch = arch::Arch<interner::DefaultInterner>;

/// Gentoo variant configuration
///
/// A variant represents a specific Gentoo system configuration combining
/// an architecture with a flavor/profile. This corresponds to Gentoo's
/// concept of system profiles and build configurations.
pub type Variant = variant::Variant<interner::DefaultInterner>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_alias() {
        let a = Arch::from_chost("aarch64").unwrap();

        println!("{a:?}");
    }
}
