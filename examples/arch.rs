//! Gentoo Architecture Demo
//!
//! Demonstrates the KnownArch enumeration and Arch<K> typed keyword.

use gentoo_core::{Arch, KnownArch};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Gentoo Architecture Demo");
    println!("=========================\n");

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

    println!("KnownArch parsing:");
    for arch_str in architectures {
        match KnownArch::parse(arch_str) {
            Ok(arch) => println!(
                "  {} -> {} (keyword: {}, bitness: {})",
                arch_str,
                arch,
                arch.as_keyword(),
                arch.bitness()
            ),
            Err(e) => println!("  {} -> Error: {}", arch_str, e),
        }
    }

    println!("\nArch<u32> global interning:");
    let exotic = Arch::intern("mymachine");
    println!(
        "  intern(\"mymachine\") -> {:?}  as_str={}",
        exotic,
        exotic.as_str()
    );
    let known = Arch::intern("amd64");
    println!(
        "  intern(\"amd64\")     -> {:?}  as_str={}",
        known,
        known.as_str()
    );

    println!("\nCHOST parsing:");
    for chost in [
        "x86_64-pc-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "powerpc64le-unknown-linux-gnu",
    ] {
        if let Some(arch) = Arch::from_chost(chost) {
            println!("  {} -> {}", chost, arch.as_str());
        }
    }

    println!("\nCurrent system architecture:");
    match KnownArch::current() {
        Ok(arch) => println!(
            "  {} (keyword: {}, bitness: {})",
            arch,
            arch.as_keyword(),
            arch.bitness()
        ),
        Err(e) => println!("  Error: {e}"),
    }

    Ok(())
}
