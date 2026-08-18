use std::cell::RefCell;
use std::collections::HashMap;

use super::error::Error;
use super::id_types::EntityId;

pub(super) struct ComponentsStorage<C: 'static> {
    /// A vector of pairs of entity id and the respective component data
    pub(super) components: Vec<(EntityId, RefCell<C>)>,
    /// A map between entity IDs and their respective component index in `components`
    pub(super) entity_id_to_component: HashMap<EntityId, usize>,
}

impl<C> ComponentsStorage<C> {
    pub(super) fn new() -> Self {
        Self {
            components: Vec::new(),
            entity_id_to_component: HashMap::new(),
        }
    }

    /// Add the `component` to this entity.
    pub(super) fn add_component(&mut self, entity_id: EntityId, component: C) -> Result<(), Error> {
        if self
            .entity_id_to_component
            .contains_key(&entity_id)
        {
            return Err(Error::ComponentAlreadyAdded(
                std::any::type_name::<C>(),
                entity_id,
            ));
        }

        let component_index = self.components.len();

        self
            .components
            .push((entity_id, RefCell::new(component)));
        self
            .entity_id_to_component
            .insert(entity_id, component_index);

        Ok(())
    }

    pub(super) fn remove_component(&mut self, entity_id: EntityId) -> Result<C, Error> {
        let entity_component_index = *self
            .entity_id_to_component
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(
                std::any::type_name::<C>(),
                entity_id,
            ))?;

        let popped_component = self
            .components
            .pop()
            .expect("There can't be no components, because there is an entity");

        let entity_component_data =
            if entity_component_index == self.components.len() {
                // The popped component is of the last entity. No need for swaps.
                popped_component.1
            } else {
                // Otherwise swap the entity's component with the component we just popped.

                // Update the entity component map to the new index
                if let Some(index) = self
                    .entity_id_to_component
                    .get_mut(&popped_component.0)
                {
                    *index = entity_component_index
                }

                std::mem::replace(
                    &mut self.components[entity_component_index],
                    popped_component,
                )
                .1
            };

        // Remove from the id-to-component map
        self.entity_id_to_component.remove(&entity_id);

        Ok(entity_component_data.into_inner())
    }

    pub(super) fn get_entity_component_index(&self, entity_id: EntityId) -> Result<usize, Error> {
        let component_index = *self
            .entity_id_to_component
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
        return self.components.len()
    }
}
