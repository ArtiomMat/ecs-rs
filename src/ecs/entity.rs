use std::hash::Hash;

// struct Entity {
//     /// Indices of the
//     component_indices: HashMap<TypeId, usize>,
// }

/// Comparing it can be useful sometimes:
///
/// - `a > b` means that `a` was allocated after `b`.
/// - `a == b` means that `a` refers to the same underlying entity as `b`.
///
/// Non-comarison traits are mostly derived for internal use, but are there for
/// your use too.
#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct EntityId(pub(super) usize);
