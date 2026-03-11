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

// ── Interned<I> ─────────────────────────────────────────────────────────

/// An interned string key parameterised by its [`Interner`] `I`.
///
/// The inner key is private. The string can be recovered via
/// [`resolve_with`](Interned::resolve_with), or through serde when
/// `I: Default` (both built-in interners implement `Default`).
///
/// `Interned<I>` is `Copy` when `<I as Interner>::Key: Copy`
/// (e.g. `u32` with [`GlobalInterner`]) and `Clone`-only otherwise
/// (e.g. `Box<str>` with [`NoInterner`]).
pub struct Interned<I: Interner>(<I as Interner>::Key);

// Manual impls to avoid spurious `I: Trait` bounds from `#[derive]`.
impl<I: Interner> Clone for Interned<I> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<I: Interner> Copy for Interned<I> where <I as Interner>::Key: Copy {}
impl<I: Interner> PartialEq for Interned<I> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<I: Interner> Eq for Interned<I> {}
impl<I: Interner> std::hash::Hash for Interned<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<I: Interner> std::fmt::Debug for Interned<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Interned").field(&self.0).finish()
    }
}

impl<I: Interner> Interned<I> {
    pub(crate) fn intern_with(s: &str, interner: &I) -> Self {
        Self(interner.get_or_intern(s))
    }

    pub(crate) fn resolve_with<'a>(&'a self, interner: &'a I) -> &'a str {
        interner.resolve(&self.0)
    }
}

/// Serializes as the interned string value regardless of `I::Key`.
#[cfg(feature = "serde")]
impl<I: Interner + Default> serde::Serialize for Interned<I> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.resolve_with(&I::default()))
    }
}

#[cfg(feature = "serde")]
impl<'de, I: Interner + Default> serde::Deserialize<'de> for Interned<I> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String as serde::Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self::intern_with(&s, &I::default()))
    }
}
