//! Gentoo architecture representation and parsing

use std::fmt;
use std::str::FromStr;

use crate::Error;

/// Gentoo architecture enumeration
///
/// Represents the architectures supported by Gentoo Linux. Each variant
/// corresponds to Gentoo's architecture keywords used in package directories
/// and profiles.
///
/// # Examples
///
/// ```
/// use gentoo_core::Arch;
///
/// // Parse architecture from string
/// let arch: Arch = "amd64".parse().unwrap();
/// assert_eq!(arch.as_keyword(), "amd64");
///
/// // Get current system architecture
/// let current_arch = Arch::current().unwrap();
/// ```
///
/// # Gentoo References
///
/// - [Gentoo Handbook: Architectures](https://wiki.gentoo.org/wiki/Handbook:Main_Page)
/// - [Gentoo Architecture Keywords](https://wiki.gentoo.org/wiki/Architecture_specific_information)
/// - [Package Manager Specification (PMS)](https://projects.gentoo.org/pms/6/pms.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Arch {
    /// ARM 32-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `arm`
    /// Supports variants: armv7, armv7a, armv7l, armv7hl
    Arm,
    /// ARM 64-bit architecture (AArch64)
    ///
    /// Corresponds to Gentoo keyword: `arm64`
    /// Supports variants: aarch64, arm64, armv8, armv8a
    AArch64,
    /// x86 32-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `x86`
    /// Supports variants: i386, i486, i586, i686
    X86,
    /// x86 64-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `amd64`
    /// Supports variants: x86_64
    X86_64,
    /// RISC-V 32-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `riscv`
    Riscv32,
    /// RISC-V 64-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `riscv`
    /// Supports variants: riscv64, riscv
    Riscv64,
    /// PowerPC 32-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `ppc`
    /// Supports variants: powerpc, ppc
    Powerpc,
    /// PowerPC 64-bit architecture
    ///
    /// Corresponds to Gentoo keyword: `ppc64`
    /// Supports variants: powerpc64, ppc64
    Powerpc64,
}

impl Arch {
    /// Convert architecture to Gentoo keyword
    ///
    /// Returns the Gentoo keyword used in package directories and profiles.
    pub fn as_keyword(&self) -> &'static str {
        match self {
            Arch::Arm => "arm",
            Arch::AArch64 => "arm64",
            Arch::X86 => "x86",
            Arch::X86_64 => "amd64",
            Arch::Riscv32 | Arch::Riscv64 => "riscv",
            Arch::Powerpc => "ppc",
            Arch::Powerpc64 => "ppc64",
        }
    }

    /// Parse architecture from string
    ///
    /// Supports common architecture names and aliases used in Gentoo.
    pub fn parse(arch: &str) -> Result<Self, Error> {
        let arch_str = arch.to_lowercase();

        match arch_str.as_str() {
            // ARM variants
            "arm" | "armv7" | "armv7a" | "armv7l" | "armv7hl" => Ok(Arch::Arm),
            // AArch64 variants
            "aarch64" | "arm64" | "armv8" | "armv8a" => Ok(Arch::AArch64),
            // x86 variants
            "x86" | "i386" | "i486" | "i586" | "i686" => Ok(Arch::X86),
            // x86_64 variants
            "x86_64" | "amd64" => Ok(Arch::X86_64),
            // RISC-V 32-bit variants
            "riscv32" => Ok(Arch::Riscv32),
            // RISC-V 64-bit variants
            "riscv64" | "riscv" => Ok(Arch::Riscv64),
            // PowerPC variants
            "powerpc" | "ppc" => Ok(Arch::Powerpc),
            "powerpc64" | "ppc64" => Ok(Arch::Powerpc64),
            _ => Err(Error::ParseError(format!("Unknown architecture: {}", arch))),
        }
    }

    /// Get the bitness (32 or 64) of the architecture
    pub fn bitness(&self) -> u32 {
        match self {
            Arch::Arm | Arch::X86 | Arch::Riscv32 | Arch::Powerpc => 32,
            Arch::AArch64 | Arch::X86_64 | Arch::Riscv64 | Arch::Powerpc64 => 64,
        }
    }

    /// Get the current system architecture from std::env::consts::ARCH
    pub fn current() -> Result<Self, Error> {
        Self::parse(std::env::consts::ARCH)
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Arch::Arm => "arm",
            Arch::AArch64 => "aarch64",
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Riscv32 => "riscv32",
            Arch::Riscv64 => "riscv64",
            Arch::Powerpc => "powerpc",
            Arch::Powerpc64 => "powerpc64",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Arch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_keywords() {
        assert_eq!(Arch::Arm.as_keyword(), "arm");
        assert_eq!(Arch::AArch64.as_keyword(), "arm64");
        assert_eq!(Arch::X86.as_keyword(), "x86");
        assert_eq!(Arch::X86_64.as_keyword(), "amd64");
        assert_eq!(Arch::Riscv32.as_keyword(), "riscv");
        assert_eq!(Arch::Riscv64.as_keyword(), "riscv");
        assert_eq!(Arch::Powerpc.as_keyword(), "ppc");
        assert_eq!(Arch::Powerpc64.as_keyword(), "ppc64");
    }

    #[test]
    fn test_arch_parsing() {
        // Test main architectures
        assert!(Arch::parse("arm").is_ok());
        assert!(Arch::parse("aarch64").is_ok());
        assert!(Arch::parse("x86").is_ok());
        assert!(Arch::parse("amd64").is_ok());
        assert!(Arch::parse("riscv64").is_ok());
        assert!(Arch::parse("powerpc64").is_ok());

        // Test aliases
        assert!(Arch::parse("armv7").is_ok());
        assert!(Arch::parse("arm64").is_ok());
        assert!(Arch::parse("i686").is_ok());
        assert!(Arch::parse("x86_64").is_ok());
        assert!(Arch::parse("ppc").is_ok());

        // Test case insensitivity
        assert!(Arch::parse("AMD64").is_ok());
        assert!(Arch::parse("Arm").is_ok());

        // Test invalid architecture
        assert!(Arch::parse("invalid").is_err());
    }

    #[test]
    fn test_arch_bitness() {
        assert_eq!(Arch::Arm.bitness(), 32);
        assert_eq!(Arch::AArch64.bitness(), 64);
        assert_eq!(Arch::X86.bitness(), 32);
        assert_eq!(Arch::X86_64.bitness(), 64);
        assert_eq!(Arch::Riscv32.bitness(), 32);
        assert_eq!(Arch::Riscv64.bitness(), 64);
        assert_eq!(Arch::Powerpc.bitness(), 32);
        assert_eq!(Arch::Powerpc64.bitness(), 64);
    }

    #[test]
    fn test_arch_display() {
        assert_eq!(Arch::Arm.to_string(), "arm");
        assert_eq!(Arch::AArch64.to_string(), "aarch64");
        assert_eq!(Arch::X86.to_string(), "x86");
        assert_eq!(Arch::X86_64.to_string(), "x86_64");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("amd64".parse::<Arch>().unwrap(), Arch::X86_64);
        assert_eq!("arm".parse::<Arch>().unwrap(), Arch::Arm);
        assert!("invalid".parse::<Arch>().is_err());
    }
}
