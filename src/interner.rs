//! Architecture string interning trait and global default implementation.

use std::sync::OnceLock;

use lasso::{Key, Spur, ThreadedRodeo};

/// Trait for interning architecture keyword strings.
///
/// Implementations map keyword strings to compact, copy-able keys and resolve
/// keys back to strings. The global default is [`GlobalArchInterner`].
pub trait ArchInterner: Send + Sync {
    /// Compact, copy-able key type returned by [`get_or_intern`](Self::get_or_intern).
    type Key: Copy + Eq + std::hash::Hash + Send + Sync + 'static;

    /// Intern `s`, returning a key that is stable for the interner's lifetime.
    fn get_or_intern(&self, s: &str) -> Self::Key;

    /// Resolve `key` back to its original string.
    fn resolve(&self, key: Self::Key) -> &str;
}

/// The default global [`ArchInterner`], backed by a process-wide [`ThreadedRodeo`].
///
/// This is a zero-sized type (ZST); all state lives in a `'static` [`OnceLock`].
/// Because the storage is `'static`, strings returned by [`resolve`](ArchInterner::resolve)
/// effectively have `'static` lifetime even though the trait signature only
/// promises `&'a str`.
///
/// The global rodeo is initialised lazily on first use.
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalArchInterner;

static GLOBAL: OnceLock<ThreadedRodeo> = OnceLock::new();

fn global() -> &'static ThreadedRodeo {
    GLOBAL.get_or_init(ThreadedRodeo::default)
}

impl ArchInterner for GlobalArchInterner {
    type Key = u32;

    fn get_or_intern(&self, s: &str) -> u32 {
        global().get_or_intern(s).into_usize() as u32
    }

    fn resolve(&self, key: u32) -> &str {
        let spur = Spur::try_from_usize(key as usize).expect("invalid arch key");
        global().resolve(&spur)
    }
}
