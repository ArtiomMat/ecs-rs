use std::collections::HashMap;

use super::id_types::EntityId;

pub enum ComponentStorageType {
    SparseSet,
    Archetypes,
}

pub(super) struct ComponentsStorage<C: 'static> {
    pub(super) component_vec: Vec<(EntityId, C)>,
    /// A map between entity IDs and their respective component index
    pub(super) entity_component_map: HashMap<EntityId, usize>,
}

impl<C> ComponentsStorage<C> {
    pub(super) fn new() -> Self {
        Self {
            component_vec: Vec::new(),
            entity_component_map: HashMap::new(),
        }
    }
}