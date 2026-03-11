//! String interning for efficient string storage.
//!
//! This module provides a flexible interning system for [`Arch`](crate::Arch)
//! and [`Variant`](crate::Variant`), reducing memory usage when processing
//! large numbers of architecture keywords (e.g., parsing an entire ebuild
//! repository's `KEYWORDS` metadata).
//!
//! # Why Interning?
//!
//! When the same string appears many times (like `"amd64"` in thousands of
//! ebuilds), interning replaces each copy with a small integer key. All
//! identical strings share one allocation, trading a small lookup cost for
//! significant memory savings.
//!
//! # Components
//!
//! - [`Interner`]: A trait defining how strings are interned and resolved.
//!   Implementations use static methods for type-level configuration.
//! - [`Interned<I>`]: A type holding an interned string key. Uses `PhantomData<I>`
//!   to associate the key with its interner without storing a reference.
//! - [`DefaultInterner`]: The default interner based on feature flags.
//!
//! # Feature Selection
//!
//! | Feature | DefaultInterner | Key Type | Behavior |
//! |---------|-----------------|----------|----------|
//! | `interner` (default) | `GlobalInterner` | `u32` | Process-global deduplication, `Copy` types |
//! | no `interner` | `NoInterner` | `Box<str>` | No deduplication, `Clone` only |
//!
//! # Usage
//!
//! The default types [`Arch`](crate::Arch) and [`Variant`](crate::Variant) use
//! `DefaultInterner` automatically:
//!
//! ```
//! use gentoo_core::Arch;
//!
//! let arch = Arch::intern("amd64");
//! assert_eq!(arch.as_str(), "amd64");
//! ```
//!
//! For custom interner implementations, use the generic types directly:
//!
//! ```
//! use gentoo_core::arch::Arch;
//! use gentoo_core::interner::NoInterner;
//!
//! let arch: Arch<NoInterner> = Arch::intern("custom");
//! assert_eq!(arch.as_str(), "custom");
//! ```
//!
//! # Custom Interners
//!
//! Implement [`Interner`] for a marker type to provide custom interning:
//!
//! ```ignore
//! use gentoo_core::interner::Interner;
//!
//! struct MyInterner;
//!
//! impl Interner for MyInterner {
//!     type Key = u64;
//!
//!     fn get_or_intern(s: &str) -> Self::Key { /* ... */ }
//!     fn resolve(key: &Self::Key) -> &str { /* ... */ }
//! }
//! ```

use std::fmt::Debug;
use std::marker::PhantomData;

/// Trait for interning strings into compact, copy-able keys.
///
/// Implementations map strings to keys and resolve keys back to strings.
/// All methods are static (no `&self`), allowing the interner type to serve
/// as a configuration parameter without carrying runtime state.
///
/// # Associated Types
///
/// - [`Key`](Self::Key): The type used to represent interned strings.
///   Must be `Clone + Eq + Hash + Send + Sync + 'static`. Small `Copy` types
///   like `u32` are preferred for efficiency, but `Box<str>` works for
///   non-deduplicating implementations.
///
/// # Example Implementations
///
/// - [`GlobalInterner`]: Process-global deduplication using `lasso` (requires `interner` feature)
/// - [`NoInterner`]: No deduplication, each string is boxed separately
pub trait Interner: Send + Sync + 'static {
    /// Key type returned by [`get_or_intern`](Self::get_or_intern).
    type Key: Clone + Eq + std::hash::Hash + Send + Sync + 'static + Debug;

    /// Intern `s`, returning a stable key.
    fn get_or_intern(s: &str) -> Self::Key;

    /// Resolve `key` back to its original string.
    fn resolve(key: &Self::Key) -> &str;
}

/// A non-interning fallback that allocates each string as a `Box<str>`.
///
/// Each call to [`get_or_intern`](Interner::get_or_intern) allocates a new
/// `Box<str>` without deduplication. Use this when the `interner` feature is
/// disabled or when simplicity is more important than memory efficiency.
///
/// The [`Key`](Interner::Key) type is `Box<str>`, making `Interned<NoInterner>`
/// `Clone` but not `Copy`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoInterner;

impl Interner for NoInterner {
    type Key = Box<str>;

    fn get_or_intern(s: &str) -> Box<str> {
        Box::from(s)
    }

    fn resolve(key: &Box<str>) -> &str {
        key
    }
}

/// The global process-wide [`Interner`], backed by a `lasso::ThreadedRodeo`.
///
/// This is a zero-sized type (ZST); all state lives in a process-wide static.
/// Strings are deduplicated across the entire process, and keys are stable
/// `u32` values.
///
/// The [`Key`](Interner::Key) type is `u32`, making `Interned<GlobalInterner>`
/// both `Clone` and `Copy`.
///
/// Requires the `interner` feature (enabled by default).
#[cfg(feature = "interner")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GlobalInterner;

#[cfg(feature = "interner")]
static GLOBAL: std::sync::OnceLock<lasso::ThreadedRodeo> = std::sync::OnceLock::new();

#[cfg(feature = "interner")]
fn global() -> &'static lasso::ThreadedRodeo {
    GLOBAL.get_or_init(lasso::ThreadedRodeo::default)
}

#[cfg(feature = "interner")]
impl Interner for GlobalInterner {
    type Key = u32;

    fn get_or_intern(s: &str) -> u32 {
        use lasso::Key as _;
        global().get_or_intern(s).into_usize() as u32
    }

    fn resolve(key: &u32) -> &str {
        use lasso::Key as _;
        let spur = lasso::Spur::try_from_usize(*key as usize).expect("invalid interner key");
        global().resolve(&spur)
    }
}

/// Default interner type based on feature configuration.
///
/// - With `interner` feature (default): [`GlobalInterner`]
/// - Without `interner` feature: [`NoInterner`]
#[cfg(feature = "interner")]
pub type DefaultInterner = GlobalInterner;
#[cfg(not(feature = "interner"))]
pub type DefaultInterner = NoInterner;

/// An interned string key parameterized by its [`Interner`] type `I`.
///
/// Holds a key of type `<I as Interner>::Key` and uses `PhantomData<I>` to
/// associate the key with its interner without storing an interner reference.
///
/// # Type Parameters
///
/// - `I`: The [`Interner`] implementation used to intern and resolve strings.
///
/// # Memory Layout
///
/// With [`GlobalInterner`], `Interned<I>` is the size of a `u32` (4 bytes).
/// With [`NoInterner`], it's the size of a `Box<str>` (a pointer).
///
/// # Serde Support
///
/// With the `serde` feature, `Interned<I>` serializes as the interned string
/// and deserializes by interning the string via `I::get_or_intern`.
pub struct Interned<I: Interner> {
    key: <I as Interner>::Key,
    _marker: PhantomData<I>,
}

impl<I: Interner> Clone for Interned<I> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _marker: PhantomData,
        }
    }
}
impl<I: Interner> Copy for Interned<I> where <I as Interner>::Key: Copy {}
impl<I: Interner> PartialEq for Interned<I> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<I: Interner> Eq for Interned<I> {}
impl<I: Interner> std::hash::Hash for Interned<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
impl<I: Interner> std::fmt::Debug for Interned<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Interned").field(&self.key).finish()
    }
}

impl<I: Interner> Interned<I> {
    /// Intern a string, returning a new `Interned<I>`.
    pub fn intern(s: &str) -> Self {
        Self {
            key: I::get_or_intern(s),
            _marker: PhantomData,
        }
    }

    /// Resolve this interned key back to its original string.
    pub fn resolve(&self) -> &str {
        I::resolve(&self.key)
    }
}

#[cfg(feature = "serde")]
impl<I: Interner> serde::Serialize for Interned<I> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.resolve())
    }
}

#[cfg(feature = "serde")]
impl<'de, I: Interner> serde::Deserialize<'de> for Interned<I> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String as serde::Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self::intern(&s))
    }
}
