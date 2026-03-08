//! Gentoo variant configuration

use crate::{Error, KnownArch};
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
/// use gentoo_core::{Variant, KnownArch};
///
/// // Parse from string format (arch-flavor)
/// let variant: Variant = "arm64-openrc".parse().unwrap();
/// assert_eq!(variant.arch, KnownArch::AArch64);
/// assert_eq!(variant.flavor, "openrc");
///
/// // Create programmatically
/// let variant = Variant::new(KnownArch::X86_64, "systemd".to_string());
/// assert_eq!(variant.to_string(), "amd64-systemd");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    /// Variant architecture.
    pub arch: KnownArch,
    /// Variant flavor/profile (e.g. `"openrc"`, `"systemd"`, `"musl-hardened"`).
    pub flavor: String,
}

impl Variant {
    /// Create a new Gentoo variant.
    pub fn new(arch: KnownArch, flavor: String) -> Self {
        Self { arch, flavor }
    }

    /// The Gentoo keyword for this variant's architecture.
    pub fn keyword(&self) -> &'static str {
        self.arch.as_keyword()
    }

    /// Parse a variant from separate architecture and flavor strings.
    pub fn parse(arch: &str, flavor: &str) -> Result<Self, Error> {
        let arch = KnownArch::parse(arch)?;
        Ok(Self::new(arch, flavor.to_string()))
    }
}

impl FromStr for Variant {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        if parts.len() != 2 {
            return Err(Error::ParseError(format!(
                "Invalid variant format: expected arch-flavor, got '{s}'"
            )));
        }
        let arch = KnownArch::parse(parts[0])?;
        Ok(Self::new(arch, parts[1].to_string()))
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
        let variant = Variant::new(KnownArch::X86_64, "systemd".to_string());
        assert_eq!(variant.arch, KnownArch::X86_64);
        assert_eq!(variant.flavor, "systemd");
    }

    #[test]
    fn test_variant_keyword() {
        assert_eq!(
            Variant::new(KnownArch::AArch64, "systemd".to_string()).keyword(),
            "arm64"
        );
        assert_eq!(
            Variant::new(KnownArch::X86, "openrc".to_string()).keyword(),
            "x86"
        );
    }

    #[test]
    fn test_variant_parsing() {
        let variant = Variant::parse("amd64", "systemd").unwrap();
        assert_eq!(variant.arch, KnownArch::X86_64);

        let variant = Variant::parse("arm", "openrc").unwrap();
        assert_eq!(variant.arch, KnownArch::Arm);

        assert!(Variant::parse("invalid", "systemd").is_err());
    }

    #[test]
    fn test_from_str() {
        let variant = "arm64-openrc".parse::<Variant>().unwrap();
        assert_eq!(variant.arch, KnownArch::AArch64);

        let variant = "amd64-musl-hardened-openrc".parse::<Variant>().unwrap();
        assert_eq!(variant.arch, KnownArch::X86_64);
        assert_eq!(variant.flavor, "musl-hardened-openrc");

        assert!("arm64".parse::<Variant>().is_err());
        assert!("invalid-openrc".parse::<Variant>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            Variant::new(KnownArch::AArch64, "openrc".to_string()).to_string(),
            "arm64-openrc"
        );
        assert_eq!(
            Variant::new(KnownArch::X86_64, "musl-hardened-openrc".to_string()).to_string(),
            "amd64-musl-hardened-openrc"
        );
    }
}
