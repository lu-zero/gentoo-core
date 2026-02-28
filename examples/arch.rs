//! Gentoo Architecture Demo
//!
//! Demonstrates the Arch enumeration and its functionality.

use gentoo_core::Arch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Gentoo Architecture Demo");
    println!("=========================\n");

    // Test parsing different architectures
    let architectures = [
        "amd64",
        "x86_64",
        "arm",
        "aarch64",
        "arm64",
        "riscv64",
        "powerpc64",
        "ppc64",
        "i686",
        "armv7",
        "riscv32",
        "powerpc",
        "ppc",
    ];

    println!("Architecture Parsing:");
    for arch_str in architectures {
        match Arch::parse(arch_str) {
            Ok(arch) => {
                println!(
                    "  {} -> {} (keyword: {}, bitness: {})",
                    arch_str,
                    arch,
                    arch.as_keyword(),
                    arch.bitness()
                );
            }
            Err(e) => {
                println!("  {} -> Error: {}", arch_str, e);
            }
        }
    }

    println!("\nKeyword Conversion:");
    let arch = Arch::parse("amd64")?;
    println!("  {} -> keyword: {}", arch, arch.as_keyword());

    let arch = Arch::parse("aarch64")?;
    println!("  {} -> keyword: {}", arch, arch.as_keyword());

    let arch = Arch::parse("riscv64")?;
    println!("  {} -> keyword: {}", arch, arch.as_keyword());

    println!("\nBitness Information:");
    let arch = Arch::parse("x86")?;
    println!("  {} is {}-bit", arch, arch.bitness());

    let arch = Arch::parse("amd64")?;
    println!("  {} is {}-bit", arch, arch.bitness());

    println!("\nDisplay Formatting:");
    let arch = Arch::parse("riscv64")?;
    println!("  String representation: {}", arch);
    println!("  Debug representation: {:?}", arch);

    println!("\nCurrent System Architecture:");
    match Arch::current() {
        Ok(current_arch) => {
            println!(
                "  Current system: {} (keyword: {}, bitness: {})",
                current_arch,
                current_arch.as_keyword(),
                current_arch.bitness()
            );
        }
        Err(e) => {
            println!("  Error getting current architecture: {}", e);
        }
    }

    Ok(())
}
