//! String interning trait and implementations.

use std::fmt::Debug;

/// Trait for interning strings into compact, copy-able keys.
///
/// Implementations map strings to keys and resolve keys back to strings.
/// The default implementation is [`GlobalInterner`] (with the `interner`
/// feature) or [`NoInterner`] for zero-dependency fallback.
pub trait Interner: Send + Sync {
    /// Key type returned by [`get_or_intern`](Self::get_or_intern).
    ///
    /// Must be `Clone + Eq + Hash`. Implementations backed by an arena
    /// (e.g. [`GlobalInterner`]) use a small `Copy` integer; others
    /// (e.g. [`NoInterner`]) use `Box<str>`.
    type Key: Clone + Eq + std::hash::Hash + Send + Sync + 'static + Debug;

    /// Intern `s`, returning a stable key valid for this interner's lifetime.
    fn get_or_intern(&self, s: &str) -> Self::Key;

    /// Resolve `key` back to its original string.
    ///
    /// The returned `&str` lifetime is tied to both `self` and `key` so that
    /// implementations like [`NoInterner`] can return data stored in the key
    /// itself rather than in a separate arena.
    fn resolve<'a>(&'a self, key: &'a Self::Key) -> &'a str;
}

/// A non-interning fallback that allocates each string as a `Box<str>`.
///
/// Each call to [`get_or_intern`](Interner::get_or_intern) allocates a new
/// `Box<str>` without deduplication. Use this when the `interner` feature is
/// disabled or when simplicity is more important than memory efficiency.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoInterner;

impl Interner for NoInterner {
    type Key = Box<str>;

    fn get_or_intern(&self, s: &str) -> Box<str> {
        Box::from(s)
    }

    fn resolve<'a>(&'a self, key: &'a Box<str>) -> &'a str {
        key
    }
}

/// The global process-wide [`Interner`], backed by a [`lasso::ThreadedRodeo`].
///
/// This is a zero-sized type (ZST); all state lives in a `'static`
/// [`std::sync::OnceLock`]. Strings returned by [`resolve`](Interner::resolve)
/// effectively have `'static` lifetime even though the signature only promises
/// the lifetime of `key`.
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

    fn get_or_intern(&self, s: &str) -> u32 {
        use lasso::Key as _;
        global().get_or_intern(s).into_usize() as u32
    }

    fn resolve<'a>(&'a self, key: &'a u32) -> &'a str {
        use lasso::Key as _;
        let spur = lasso::Spur::try_from_usize(*key as usize).expect("invalid interner key");
        global().resolve(&spur)
    }
}

#[cfg(feature = "interner")]
pub type DefaultInterner = GlobalInterner;
#[cfg(not(feature = "interner"))]
pub type DefaultInterner = NoInterner;
