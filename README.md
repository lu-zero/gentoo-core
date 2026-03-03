# gentoo-core

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/gentoo-core.svg)](https://crates.io/crates/gentoo-core)
[![docs.rs](https://docs.rs/gentoo-core/badge.svg)](https://docs.rs/gentoo-core)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/lu-zero/gentoo-core/actions/workflows/ci.yml/badge.svg)](https://github.com/lu-zero/gentoo-core/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/lu-zero/gentoo-core/graph/badge.svg?token=fApuKCrcgU)](https://codecov.io/github/lu-zero/gentoo-core)

Core Gentoo types and utilities for Rust applications.


## Overview

`gentoo-core` provides fundamental Gentoo-specific types and utilities that can be used across various Gentoo-related Rust projects.

## Features

- Gentoo architecture representation and parsing
- Variant configuration for Gentoo systems

## Architecture Support

The crate supports the following Gentoo architectures:

- `arm`, `aarch64` (arm64)
- `x86`, `amd64` (x86_64)
- `riscv`, `riscv64`
- `powerpc`, `ppc64`
- And their Gentoo keyword equivalents

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gentoo-core = "0.1"
```

## Usage

```rust
use gentoo_core::{Arch, Variant};

// Parse an architecture
let arch = Arch::parse("amd64").unwrap();
println!("Arch: {} (keyword: {})", arch, arch.as_keyword());

// Create a variant configuration
let variant = Variant::new(arch, "systemd".to_string());
println!("Variant: {:?}", variant);
```

## Examples

Run the included examples:

```bash
cargo run --example arch_demo
```

## Contributing

See [AGENTS.md](AGENTS.md) for project conventions and contribution guidelines.

## License

MIT
