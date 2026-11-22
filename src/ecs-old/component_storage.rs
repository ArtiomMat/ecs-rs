use std::cell::RefCell;
use std::collections::HashMap;

use super::error::Error;
use super::entity_id::EntityId;

pub(super) struct ComponentStorageItem<C> {
    pub(super) component: RefCell<C>,
    pub(super) entity_id: EntityId,
}

pub(super) struct ComponentsStorage<C: 'static> {
    /// Each index here maps to an ID in `entity_ids`.
    pub(super) items: Vec<ComponentStorageItem<C>>,
    /// A map between entity IDs and their respective component index in `components`
    pub(super) entity_id_to_item: HashMap<EntityId, usize>,
}

impl<C> ComponentsStorage<C> {
    pub(super) fn new() -> Self {
        Self {
            items: Vec::new(),
            entity_id_to_item: HashMap::new(),
        }
    }

    /// Add the `component` to this entity.
    pub(super) fn add_component(&mut self, entity_id: EntityId, component: C) -> Result<(), Error> {
        if self.entity_id_to_item.contains_key(&entity_id) {
            return Err(Error::ComponentAlreadyAdded(
                std::any::type_name::<C>(),
                entity_id,
            ));
        }

        let component_index = self.items.len();

        self.items.push(ComponentStorageItem {
            component: RefCell::new(component),
            entity_id: entity_id
        });
        self.entity_id_to_item
            .insert(entity_id, component_index);

        Ok(())
    }

    pub(super) fn remove_component(&mut self, entity_id: EntityId) -> Result<C, Error> {
        let entity_item_index =
            *self
                .entity_id_to_item
                .get(&entity_id)
                .ok_or(Error::InvalidEntityComponent(
                    std::any::type_name::<C>(),
                    entity_id,
                ))?;

        // Before we remove_swap, ensure we update `entity_id_to_item` to now
        // point to the swapped element.
        let swapped_entity_id = self.items.last().unwrap().entity_id;
        if let Some(index) = self.entity_id_to_item.get_mut(&swapped_entity_id) {
            *index = entity_item_index
        }

        let entity_item = self.items.swap_remove(entity_item_index);

        // Remove this entity's id from the map
        self.entity_id_to_item.remove(&entity_id);

        Ok(entity_item.component.into_inner())
    }

    pub(super) fn get_entity_component_index(&self, entity_id: EntityId) -> Result<usize, Error> {
        let component_index =
            *self
                .entity_id_to_item
                .get(&entity_id)
                .ok_or(Error::InvalidEntityComponent(
                    std::any::type_name::<C>(),
                    entity_id,
                ))?;

        Ok(component_index)
    }

    /// Returns how many component instances it stores, i.e. how many instances
    /// have `C` registered to them.
    pub(super) fn len(&self) -> usize {
        return self.items.len();
    }
}
