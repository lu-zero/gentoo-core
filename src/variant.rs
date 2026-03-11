//! Gentoo variant configuration

use crate::arch::Arch;
use crate::error::Error;
use crate::interner::{DefaultInterner, Interned, Interner};
use std::fmt;
use std::str::FromStr;

// ── Variant<I> ────────────────────────────────────────────────────────────────

/// Gentoo variant configuration
///
/// A variant represents a specific Gentoo system configuration combining
/// an architecture with a flavor/profile. This corresponds to Gentoo's
/// concept of system profiles and build configurations.
///
/// `Variant<I>` is generic over an [`Interner`] type parameter that controls
/// how both the arch keyword and flavor string are stored.  The default,
/// [`DefaultInterner`], uses the global interner (with the `interner` feature)
/// or inline heap allocation without it.
///
/// # Examples
///
/// ```
/// use gentoo_core::{Variant, Arch, KnownArch};
///
/// // Parse from string format (arch-flavor)
/// let variant: Variant = "arm64-openrc".parse().unwrap();
/// assert!(matches!(variant.arch, Arch::Known(KnownArch::AArch64)));
/// assert_eq!(variant.flavor(), "openrc");
/// ```
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = ""))]
pub struct Variant<I = DefaultInterner>
where
    I: Interner,
{
    /// Variant architecture.
    pub arch: Arch<I>,
    /// Interned flavor/profile string (e.g. `"openrc"`, `"systemd"`).
    flavor: Interned<I>,
}

impl<I: Interner> Clone for Variant<I> {
    fn clone(&self) -> Self {
        Self {
            arch: self.arch.clone(),
            flavor: self.flavor.clone(),
        }
    }
}

impl<I: Interner> Copy for Variant<I> where Interned<I>: Copy {}

impl<I: Interner> Variant<I> {
    /// Create a variant from an arch and a flavor string using interner `I`.
    pub(crate) fn new(arch: Arch<I>, flavor: &str) -> Self {
        Self {
            arch,
            flavor: Interned::intern(flavor),
        }
    }

    /// Parse arch + flavor strings using the interner `I`.
    pub fn parse(arch: &str, flavor: &str) -> Result<Self, Error> {
        let arch = Arch::intern(arch);
        Ok(Self::new(arch, flavor))
    }

    /// Resolve the flavor string using the interner `I`.
    pub fn flavor(&self) -> &str {
        self.flavor.resolve()
    }

    /// The Gentoo keyword for this variant's architecture.
    pub fn keyword(&self) -> &str {
        self.arch.as_str()
    }
}

impl<I: Interner> fmt::Display for Variant<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.arch.as_str(), self.flavor())
    }
}

impl<I: Interner> FromStr for Variant<I> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (arch_str, flavor_str) = s.split_once('-').ok_or_else(|| {
            Error::ParseError(format!(
                "Invalid variant format: expected arch-flavor, got '{s}'"
            ))
        })?;
        Self::parse(arch_str, flavor_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::KnownArch;

    #[test]
    fn test_variant_creation() {
        let variant: Variant = Variant::new(Arch::Known(KnownArch::X86_64), "systemd");
        assert_eq!(variant.arch, Arch::Known(KnownArch::X86_64));
        assert_eq!(variant.flavor(), "systemd");
    }

    #[test]
    fn test_variant_keyword() {
        assert_eq!(
            Variant::new(<Arch>::Known(KnownArch::AArch64), "systemd").keyword(),
            "arm64"
        );
        assert_eq!(
            Variant::new(<Arch>::Known(KnownArch::X86), "openrc").keyword(),
            "x86"
        );
    }

    #[test]
    fn test_variant_parsing() {
        let variant: Variant = Variant::parse("amd64", "systemd").unwrap();
        assert_eq!(variant.arch, Arch::Known(KnownArch::X86_64));

        let variant: Variant = Variant::parse("arm", "openrc").unwrap();
        assert_eq!(variant.arch, Arch::Known(KnownArch::Arm));
    }

    #[test]
    fn test_from_str() {
        let variant = "arm64-openrc".parse::<Variant>().unwrap();
        assert!(matches!(variant.arch, Arch::Known(KnownArch::AArch64)));

        let variant = "amd64-musl-hardened-openrc".parse::<Variant>().unwrap();
        assert_eq!(variant.arch, Arch::Known(KnownArch::X86_64));
        assert_eq!(variant.flavor(), "musl-hardened-openrc");

        assert!("arm64".parse::<Variant>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            Variant::new(<Arch>::Known(KnownArch::AArch64), "openrc").to_string(),
            "arm64-openrc"
        );
        assert_eq!(
            Variant::new(<Arch>::Known(KnownArch::X86_64), "musl-hardened-openrc").to_string(),
            "amd64-musl-hardened-openrc"
        );
    }

    // ── Serde roundtrip ──────────────────────────────────────────────────────

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn variant_known_arch_serializes_as_strings() {
            let variant: Variant = "amd64-systemd".parse().unwrap();
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, r#"{"arch":"amd64","flavor":"systemd"}"#);
        }

        #[test]
        fn variant_known_arch_roundtrip() {
            let original: Variant = "amd64-systemd".parse().unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Variant = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
            assert_eq!(restored.flavor(), "systemd");
        }

        #[test]
        fn variant_exotic_arch_roundtrip() {
            let original: Variant = "mymachine-openrc".parse().unwrap();
            let json = serde_json::to_string(&original).unwrap();
            assert_eq!(json, r#"{"arch":"mymachine","flavor":"openrc"}"#);
            let restored: Variant = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }

        #[test]
        fn variant_complex_flavor_roundtrip() {
            let original: Variant = "amd64-musl-hardened-openrc".parse().unwrap();
            let json = serde_json::to_string(&original).unwrap();
            assert_eq!(json, r#"{"arch":"amd64","flavor":"musl-hardened-openrc"}"#);
            let restored: Variant = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }
    }
}
