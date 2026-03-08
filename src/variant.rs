//! Gentoo variant configuration

#[cfg(feature = "interner")]
use crate::interner::GlobalInterner;
#[cfg(not(feature = "interner"))]
use crate::interner::NoInterner;
use crate::interner::{DefaultInterner, Interner};
use crate::{Arch, Error};
use std::fmt;
use std::str::FromStr;

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
///
/// // Create programmatically
/// let variant = Variant::new(Arch::Known(KnownArch::X86_64), "systemd");
/// assert_eq!(variant.to_string(), "amd64-systemd");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "I::Key: serde::Serialize + for<'de2> serde::Deserialize<'de2>")
)]
pub struct Variant<I = DefaultInterner>
where
    I: Interner,
{
    /// Variant architecture.
    pub arch: Arch<I>,
    /// Interned flavor/profile string key (e.g. `"openrc"`, `"systemd"`).
    flavor: <I as Interner>::Key,
}

impl<I: Interner> Variant<I> {
    /// Create a variant from an already-interned arch and a flavor string.
    pub fn new_with(arch: Arch<I>, flavor: &str, interner: &I) -> Self {
        Self {
            arch,
            flavor: interner.get_or_intern(flavor),
        }
    }

    /// Parse arch + flavor strings using the given interner.
    pub fn parse_with(arch: &str, flavor: &str, interner: &I) -> Result<Self, Error> {
        let arch = Arch::intern_with(arch, interner);
        Ok(Self::new_with(arch, flavor, interner))
    }

    /// Resolve the flavor string using the given interner.
    pub fn flavor_with<'a>(&'a self, interner: &'a I) -> &'a str {
        interner.resolve(&self.flavor)
    }

    /// The Gentoo keyword for this variant's architecture.
    pub fn keyword_with<'a>(&'a self, interner: &'a I) -> &'a str {
        self.arch.resolve_with(interner)
    }
}

#[cfg(feature = "interner")]
impl Variant<GlobalInterner> {
    /// Create a variant using the global interner.
    pub fn new(arch: Arch<GlobalInterner>, flavor: &str) -> Self {
        Self::new_with(arch, flavor, &GlobalInterner)
    }

    /// Parse arch + flavor strings using the global interner.
    pub fn parse(arch: &str, flavor: &str) -> Result<Self, Error> {
        Self::parse_with(arch, flavor, &GlobalInterner)
    }

    /// Resolve the flavor string using the global interner.
    pub fn flavor(&self) -> &str {
        self.flavor_with(&GlobalInterner)
    }

    /// The Gentoo keyword for this variant's architecture.
    pub fn keyword(&self) -> &str {
        self.keyword_with(&GlobalInterner)
    }
}

#[cfg(not(feature = "interner"))]
impl Variant<NoInterner> {
    /// Create a variant using inline allocation.
    pub fn new(arch: Arch<NoInterner>, flavor: &str) -> Self {
        Self::new_with(arch, flavor, &NoInterner)
    }

    /// Parse arch + flavor strings using inline allocation.
    pub fn parse(arch: &str, flavor: &str) -> Result<Self, Error> {
        Self::parse_with(arch, flavor, &NoInterner)
    }

    /// Return the flavor string.
    pub fn flavor(&self) -> &str {
        self.flavor_with(&NoInterner)
    }

    /// The Gentoo keyword for this variant's architecture.
    pub fn keyword(&self) -> &str {
        self.keyword_with(&NoInterner)
    }
}

impl<I: Interner + Default> fmt::Display for Variant<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let interner = I::default();
        write!(
            f,
            "{}-{}",
            self.arch.resolve_with(&interner),
            self.flavor_with(&interner)
        )
    }
}

impl<I: Interner + Default> FromStr for Variant<I> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (arch_str, flavor_str) = s.split_once('-').ok_or_else(|| {
            Error::ParseError(format!(
                "Invalid variant format: expected arch-flavor, got '{s}'"
            ))
        })?;
        Self::parse_with(arch_str, flavor_str, &I::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KnownArch;

    #[test]
    fn test_variant_creation() {
        let variant = Variant::new(Arch::Known(KnownArch::X86_64), "systemd");
        assert_eq!(variant.arch, Arch::Known(KnownArch::X86_64));
        assert_eq!(variant.flavor(), "systemd");
    }

    #[test]
    fn test_variant_keyword() {
        assert_eq!(
            Variant::new(Arch::Known(KnownArch::AArch64), "systemd").keyword(),
            "arm64"
        );
        assert_eq!(
            Variant::new(Arch::Known(KnownArch::X86), "openrc").keyword(),
            "x86"
        );
    }

    #[test]
    fn test_variant_parsing() {
        let variant = Variant::parse("amd64", "systemd").unwrap();
        assert_eq!(variant.arch, Arch::Known(KnownArch::X86_64));

        let variant = Variant::parse("arm", "openrc").unwrap();
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
            Variant::new(Arch::Known(KnownArch::AArch64), "openrc").to_string(),
            "arm64-openrc"
        );
        assert_eq!(
            Variant::new(Arch::Known(KnownArch::X86_64), "musl-hardened-openrc").to_string(),
            "amd64-musl-hardened-openrc"
        );
    }
}
