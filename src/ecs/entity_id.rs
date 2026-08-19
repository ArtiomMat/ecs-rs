use std::any::TypeId;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq)]
pub struct EntityId(pub(super) usize);

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{}", self.0)
    }
}
