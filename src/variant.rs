//! Gentoo variant configuration

use crate::{Arch, Error};
use std::fmt;
use std::str::FromStr;

/// Gentoo variant configuration
///
/// A variant represents a specific Gentoo system configuration combining
/// an architecture with a flavor/profile. This corresponds to Gentoo's
/// concept of system profiles and build configurations.
///
/// # Examples
///
/// ```
/// use gentoo_core::{Variant, Arch};
///
/// // Parse from string format (arch-flavor)
/// let variant: Variant = "arm64-openrc".parse().unwrap();
/// assert_eq!(variant.arch, Arch::AArch64);
/// assert_eq!(variant.flavor, "openrc");
///
/// // Create programmatically
/// let variant = Variant::new(Arch::X86_64, "systemd".to_string());
/// assert_eq!(variant.to_string(), "amd64-systemd");
/// ```
///
/// # Gentoo References
///
/// - [Gentoo Handbook: Architectures](https://wiki.gentoo.org/wiki/Handbook:Main_Page)
/// - [Gentoo Profiles](https://wiki.gentoo.org/wiki/Profile_(Portage))
/// - [Package Manager Specification (PMS)](https://projects.gentoo.org/pms/6/pms.html)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Variant {
    /// Variant architecture
    ///
    /// See [Arch] enum for supported architectures and their Gentoo keywords.
    pub arch: Arch,
    /// Variant flavor/profile
    ///
    /// The flavor represents the system profile or configuration variant.
    /// Common examples include "openrc", "systemd", "musl", "hardened",
    /// and combinations like "musl-hardened-openrc".
    pub flavor: String,
}

impl Variant {
    /// Create a new Gentoo variant
    pub fn new(arch: Arch, flavor: String) -> Self {
        Self { arch, flavor }
    }

    /// Get the Gentoo keyword for this variant
    pub fn keyword(&self) -> &'static str {
        self.arch.as_keyword()
    }

    /// Parse variant from architecture string and flavor
    pub fn parse(arch: &str, flavor: &str) -> Result<Self, Error> {
        let arch = Arch::parse(arch)?;
        Ok(Self::new(arch, flavor.to_string()))
    }
}

impl FromStr for Variant {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split on the first hyphen to separate arch from flavor
        // This allows flavors to contain hyphens (e.g., "musl-hardened-openrc")
        let parts: Vec<&str> = s.splitn(2, '-').collect();

        if parts.len() != 2 {
            return Err(Error::ParseError(format!(
                "Invalid variant format: expected arch-flavor, got '{}'",
                s
            )));
        }

        let arch_str = parts[0];
        let flavor_str = parts[1];

        let arch = Arch::parse(arch_str)?;
        Ok(Self::new(arch, flavor_str.to_string()))
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.arch.as_keyword(), self.flavor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_creation() {
        let arch = Arch::X86_64;
        let variant = Variant::new(arch, "systemd".to_string());
        assert_eq!(variant.arch, Arch::X86_64);
        assert_eq!(variant.flavor, "systemd");
    }

    #[test]
    fn test_variant_keyword() {
        let variant = Variant::new(Arch::AArch64, "systemd".to_string());
        assert_eq!(variant.keyword(), "arm64");

        let variant = Variant::new(Arch::X86, "openrc".to_string());
        assert_eq!(variant.keyword(), "x86");
    }

    #[test]
    fn test_variant_parsing() {
        let variant = Variant::parse("amd64", "systemd").unwrap();
        assert_eq!(variant.arch, Arch::X86_64);
        assert_eq!(variant.flavor, "systemd");

        let variant = Variant::parse("arm", "openrc").unwrap();
        assert_eq!(variant.arch, Arch::Arm);
        assert_eq!(variant.flavor, "openrc");

        // Test invalid architecture
        assert!(Variant::parse("invalid", "systemd").is_err());
    }

    #[test]
    fn test_variant_equality() {
        let variant1 = Variant::new(Arch::X86_64, "systemd".to_string());
        let variant2 = Variant::new(Arch::X86_64, "systemd".to_string());
        assert_eq!(variant1, variant2);

        let variant3 = Variant::new(Arch::X86_64, "openrc".to_string());
        assert_ne!(variant1, variant3);
    }

    #[test]
    fn test_from_str() {
        // Test simple arch-flavor format
        let variant = "arm64-openrc".parse::<Variant>().unwrap();
        assert_eq!(variant.arch, Arch::AArch64);
        assert_eq!(variant.flavor, "openrc");

        // Test flavor with hyphens
        let variant = "amd64-musl-hardened-openrc".parse::<Variant>().unwrap();
        assert_eq!(variant.arch, Arch::X86_64);
        assert_eq!(variant.flavor, "musl-hardened-openrc");

        // Test invalid format (no hyphen)
        assert!("arm64".parse::<Variant>().is_err());

        // Test invalid architecture
        assert!("invalid-openrc".parse::<Variant>().is_err());
    }

    #[test]
    fn test_display() {
        let variant = Variant::new(Arch::AArch64, "openrc".to_string());
        assert_eq!(variant.to_string(), "arm64-openrc");

        let variant = Variant::new(Arch::X86_64, "musl-hardened-openrc".to_string());
        assert_eq!(variant.to_string(), "amd64-musl-hardened-openrc");

        let variant = Variant::new(Arch::X86, "systemd".to_string());
        assert_eq!(variant.to_string(), "x86-systemd");
    }
}
