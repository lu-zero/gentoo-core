//! Gentoo architecture representation and parsing.

use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

use crate::Error;
use crate::interner::{ArchInterner, GlobalArchInterner};

/// Well-known Gentoo CPU architecture variant.
///
/// Represents the 18 architectures supported by Gentoo Linux. Each variant
/// maps to a Gentoo keyword string via [`KnownArch::as_keyword`].
///
/// # Examples
///
/// ```
/// use gentoo_core::KnownArch;
///
/// let arch: KnownArch = "amd64".parse().unwrap();
/// assert_eq!(arch.as_keyword(), "amd64");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum KnownArch {
    Arm,
    AArch64,
    X86,
    X86_64,
    Riscv32,
    Riscv64,
    Powerpc,
    Powerpc64,
    Mips,
    Mips64,
    Sparc,
    Sparc64,
    S390x,
    M68k,
    LoongArch64,
    Alpha,
    Hppa,
    Ia64,
}

impl KnownArch {
    /// Gentoo keyword string for this architecture (e.g. `"amd64"`).
    pub fn as_keyword(&self) -> &'static str {
        match self {
            KnownArch::Arm => "arm",
            KnownArch::AArch64 => "arm64",
            KnownArch::X86 => "x86",
            KnownArch::X86_64 => "amd64",
            KnownArch::Riscv32 | KnownArch::Riscv64 => "riscv",
            KnownArch::Powerpc => "ppc",
            KnownArch::Powerpc64 => "ppc64",
            KnownArch::Mips | KnownArch::Mips64 => "mips",
            KnownArch::Sparc | KnownArch::Sparc64 => "sparc",
            KnownArch::S390x => "s390",
            KnownArch::M68k => "m68k",
            KnownArch::LoongArch64 => "loong",
            KnownArch::Alpha => "alpha",
            KnownArch::Hppa => "hppa",
            KnownArch::Ia64 => "ia64",
        }
    }

    /// Parse from a keyword or common alias string (case-insensitive).
    pub fn parse(arch: &str) -> Result<Self, Error> {
        match arch.to_lowercase().as_str() {
            "arm" | "armv7" | "armv7a" | "armv7l" | "armv7hl" => Ok(KnownArch::Arm),
            "aarch64" | "arm64" | "armv8" | "armv8a" => Ok(KnownArch::AArch64),
            "x86" | "i386" | "i486" | "i586" | "i686" => Ok(KnownArch::X86),
            "x86_64" | "amd64" => Ok(KnownArch::X86_64),
            "riscv32" => Ok(KnownArch::Riscv32),
            "riscv64" | "riscv" => Ok(KnownArch::Riscv64),
            "powerpc" | "ppc" => Ok(KnownArch::Powerpc),
            "powerpc64" | "ppc64" => Ok(KnownArch::Powerpc64),
            "mips" => Ok(KnownArch::Mips),
            "mips64" => Ok(KnownArch::Mips64),
            "sparc" => Ok(KnownArch::Sparc),
            "sparc64" => Ok(KnownArch::Sparc64),
            "s390" | "s390x" => Ok(KnownArch::S390x),
            "m68k" => Ok(KnownArch::M68k),
            "loong" | "loongarch64" => Ok(KnownArch::LoongArch64),
            "alpha" => Ok(KnownArch::Alpha),
            "hppa" => Ok(KnownArch::Hppa),
            "ia64" => Ok(KnownArch::Ia64),
            _ => Err(Error::ParseError(format!("Unknown architecture: {arch}"))),
        }
    }

    /// Bitness (32 or 64) of this architecture.
    pub fn bitness(&self) -> u32 {
        match self {
            KnownArch::Arm
            | KnownArch::X86
            | KnownArch::Riscv32
            | KnownArch::Powerpc
            | KnownArch::Mips
            | KnownArch::Sparc
            | KnownArch::M68k
            | KnownArch::Hppa => 32,
            KnownArch::AArch64
            | KnownArch::X86_64
            | KnownArch::Riscv64
            | KnownArch::Powerpc64
            | KnownArch::Mips64
            | KnownArch::Sparc64
            | KnownArch::S390x
            | KnownArch::LoongArch64
            | KnownArch::Alpha
            | KnownArch::Ia64 => 64,
        }
    }

    /// Current system architecture from [`std::env::consts::ARCH`].
    pub fn current() -> Result<Self, Error> {
        Self::parse(std::env::consts::ARCH)
    }
}

impl fmt::Display for KnownArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            KnownArch::Arm => "arm",
            KnownArch::AArch64 => "aarch64",
            KnownArch::X86 => "x86",
            KnownArch::X86_64 => "x86_64",
            KnownArch::Riscv32 => "riscv32",
            KnownArch::Riscv64 => "riscv64",
            KnownArch::Powerpc => "powerpc",
            KnownArch::Powerpc64 => "powerpc64",
            KnownArch::Mips => "mips",
            KnownArch::Mips64 => "mips64",
            KnownArch::Sparc => "sparc",
            KnownArch::Sparc64 => "sparc64",
            KnownArch::S390x => "s390x",
            KnownArch::M68k => "m68k",
            KnownArch::LoongArch64 => "loongarch64",
            KnownArch::Alpha => "alpha",
            KnownArch::Hppa => "hppa",
            KnownArch::Ia64 => "ia64",
        };
        write!(f, "{name}")
    }
}

impl FromStr for KnownArch {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

// ── Generic Arch<K> ──────────────────────────────────────────────────────────

/// Opaque key for an overlay-defined architecture keyword.
///
/// Wraps the interner's native key type `K`. The inner value is private so
/// the interner stays an implementation detail; only the owning interner (via
/// [`Arch::resolve_with`] or [`Arch::as_str`]) can turn it back into a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExoticKey<K>(K);

/// A Gentoo architecture keyword, either well-known or overlay-defined.
///
/// `Known` maps to [`KnownArch`] (zero-cost, `Copy`).
/// `Exotic` holds an [`ExoticKey<K>`] that must be resolved via the same
/// [`ArchInterner`] that created it.
///
/// The default key type `K = u32` matches [`GlobalArchInterner`]; callers
/// using a custom interner supply their own `K`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch<K = u32>
where
    K: Copy + Eq + Hash,
{
    /// A well-known Gentoo architecture keyword.
    Known(KnownArch),
    /// An overlay-defined architecture keyword interned in `K`-keyed storage.
    Exotic(ExoticKey<K>),
}

impl<K: Copy + Eq + Hash> Arch<K> {
    /// Intern `keyword` using `interner`.
    pub fn intern_with(keyword: &str, interner: &impl ArchInterner<Key = K>) -> Self {
        if let Ok(known) = KnownArch::parse(keyword) {
            Self::Known(known)
        } else {
            Self::Exotic(ExoticKey(interner.get_or_intern(keyword)))
        }
    }

    /// Extract the CPU arch from a GNU CHOST triple using `interner`.
    ///
    /// Returns `None` only when `chost` is empty.
    pub fn from_chost_with(chost: &str, interner: &impl ArchInterner<Key = K>) -> Option<Self> {
        let cpu = chost.split('-').next().filter(|s| !s.is_empty())?;
        Some(Self::intern_with(&normalize_chost_cpu(cpu), interner))
    }

    /// Resolve to the Gentoo keyword string using `interner`.
    pub fn resolve_with<'a>(&self, interner: &'a impl ArchInterner<Key = K>) -> &'a str {
        match self {
            Self::Known(arch) => arch.as_keyword(),
            Self::Exotic(ExoticKey(key)) => interner.resolve(*key),
        }
    }
}

/// Convenience methods using the global [`GlobalArchInterner`] (`K = u32`).
impl Arch<u32> {
    /// Intern `keyword` using the global interner.
    pub fn intern(keyword: &str) -> Self {
        Self::intern_with(keyword, &GlobalArchInterner)
    }

    /// Extract the CPU arch from a GNU CHOST triple using the global interner.
    ///
    /// Returns `None` only when `chost` is empty.
    pub fn from_chost(chost: &str) -> Option<Self> {
        Self::from_chost_with(chost, &GlobalArchInterner)
    }

    /// Resolve to the Gentoo keyword string using the global interner.
    pub fn as_str(&self) -> &str {
        self.resolve_with(&GlobalArchInterner)
    }
}

impl fmt::Display for Arch<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<str> for Arch<u32> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Arch<u32> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for Arch<u32> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

/// Normalise the CPU field of a GNU CHOST triple before matching known arches.
fn normalize_chost_cpu(cpu: &str) -> String {
    let s = cpu.to_lowercase();

    // powerpc64le / powerpc64be → powerpc64
    for suffix in &["le", "be"] {
        if let Some(base) = s.strip_suffix(suffix)
            && base == "powerpc64"
        {
            return base.to_string();
        }
    }

    // mipsel / mipseb → mips;  mips64el / mips64eb → mips64
    for suffix in &["el", "eb"] {
        if let Some(base) = s.strip_suffix(suffix)
            && (base == "mips" || base == "mips64")
        {
            return base.to_string();
        }
    }

    // riscv64gc, riscv64imac → riscv64;  riscv32gc → riscv32
    if let Some(after_riscv) = s.strip_prefix("riscv") {
        if let Some(end) = after_riscv.find(|c: char| !c.is_ascii_digit())
            && end > 0
        {
            return format!("riscv{}", &after_riscv[..end]);
        }
        return s;
    }

    // hppa2.0w, hppa1.1 → hppa
    if s.starts_with("hppa") && s.len() > "hppa".len() {
        return "hppa".to_string();
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── KnownArch ────────────────────────────────────────────────────────────

    #[test]
    fn known_arch_keywords() {
        assert_eq!(KnownArch::Arm.as_keyword(), "arm");
        assert_eq!(KnownArch::AArch64.as_keyword(), "arm64");
        assert_eq!(KnownArch::X86.as_keyword(), "x86");
        assert_eq!(KnownArch::X86_64.as_keyword(), "amd64");
        assert_eq!(KnownArch::Riscv32.as_keyword(), "riscv");
        assert_eq!(KnownArch::Riscv64.as_keyword(), "riscv");
        assert_eq!(KnownArch::Powerpc.as_keyword(), "ppc");
        assert_eq!(KnownArch::Powerpc64.as_keyword(), "ppc64");
        assert_eq!(KnownArch::LoongArch64.as_keyword(), "loong");
        assert_eq!(KnownArch::Hppa.as_keyword(), "hppa");
    }

    #[test]
    fn known_arch_parsing() {
        assert!(KnownArch::parse("arm").is_ok());
        assert!(KnownArch::parse("amd64").is_ok());
        assert!(KnownArch::parse("AMD64").is_ok());
        assert!(KnownArch::parse("invalid").is_err());
    }

    #[test]
    fn known_arch_from_str() {
        assert_eq!("amd64".parse::<KnownArch>().unwrap(), KnownArch::X86_64);
        assert!("invalid".parse::<KnownArch>().is_err());
    }

    // ── Arch<u32> global convenience ─────────────────────────────────────────

    #[test]
    fn arch_intern_known() {
        assert!(matches!(Arch::intern("amd64"), Arch::Known(_)));
        assert!(matches!(Arch::intern("arm64"), Arch::Known(_)));
        assert!(matches!(Arch::intern("loong"), Arch::Known(_)));
        assert!(matches!(Arch::intern("hppa"), Arch::Known(_)));
    }

    #[test]
    fn arch_intern_exotic() {
        let a1 = Arch::intern("mymachine");
        assert!(matches!(a1, Arch::Exotic(_)));
        assert_eq!(Arch::intern("mymachine"), a1); // same key
        assert_eq!(a1.as_str(), "mymachine");
    }

    #[test]
    fn arch_from_chost_known() {
        let cases = [
            ("x86_64-pc-linux-gnu", "amd64"),
            ("aarch64-unknown-linux-gnu", "arm64"),
            ("i686-pc-linux-gnu", "x86"),
            ("powerpc-unknown-linux-gnu", "ppc"),
            ("s390x-linux-gnu", "s390"),
        ];
        for (chost, expected) in cases {
            let arch = Arch::from_chost(chost).unwrap();
            assert_eq!(arch.as_str(), expected, "chost={chost}");
            assert!(
                matches!(arch, Arch::Known(_)),
                "chost={chost} should be Known"
            );
        }
    }

    #[test]
    fn arch_chost_normalization() {
        let cases = [
            ("powerpc64le-unknown-linux-gnu", "ppc64"),
            ("riscv64gc-unknown-linux-gnu", "riscv"),
            ("mipsel-unknown-linux-gnu", "mips"),
            ("mips64el-unknown-linux-gnu", "mips"),
            ("hppa2.0w-hp-linux-gnu", "hppa"),
        ];
        for (chost, expected) in cases {
            assert_eq!(
                Arch::from_chost(chost).unwrap().as_str(),
                expected,
                "chost={chost}"
            );
        }
    }

    #[test]
    fn arch_empty_chost() {
        assert!(Arch::from_chost("").is_none());
    }
}
