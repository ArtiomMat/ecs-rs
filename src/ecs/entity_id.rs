use std::any::TypeId;
use std::fmt::Display;
use std::hash::Hash;

/// Comparing it can be useful sometimes:
///
/// - `a > b` means that `a` was allocated after `b`.
/// - `a == b` means that `a` refers to the same underlying entity as `b`.
///
/// Non-comarison traits are mostly derived for internal use, but are there for
/// your use too.
#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct EntityId {
    pub(super) index: u32,
    pub(super) generation: u32,
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B{}G{}", self.index, self.generation)
    }
}
