use std::any::TypeId;
use std::hash::Hash;

/// Comparing it can be useful sometimes:
///
/// - `a > b` means that `a` was allocated after `b`.
/// - `a == b` means that `a` refers to the same underlying entity as `b`.
///
/// Non-comarison traits are mostly derived for internal use, but are there for
/// your use too.
#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct EntityId(pub(super) usize);

/// You may say: "Bruh, you just wrapped TypeId with a different name."
/// You are 101% correct, and I don't care, one + is that it's a unified API.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(super) struct ComponentId(pub(super) TypeId);

impl ComponentId {
    pub fn of<C: 'static>() -> Self {
        Self(std::any::TypeId::of::<C>())
    }
}

