use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::id_types::EntityId;
use super::error::Error;
use super::component_storage::ComponentsStorage;

pub struct World {
    component_storage_vecs: HashMap<TypeId, Box<dyn Any>>,
    // component_vecs: HashMap<TypeId, Box<dyn Any>>,
    // entities: HashMap<EntityId, Entity>,
    entity_validity_set: HashSet<EntityId>,
    entity_counter: AtomicUsize,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_storage_vecs: HashMap::new(),
            entity_validity_set: HashSet::new(),
            entity_counter: 0.into(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = EntityId(self.entity_counter.fetch_add(1, Ordering::Relaxed));
        self.entity_validity_set.insert(entity_id);
        entity_id
    }

    pub fn is_entity_valid(&self, id: EntityId) -> bool {
        self.entity_validity_set.contains(&id)
    }

    pub fn get_entity_component<C: 'static>(&self, entity_id: EntityId) -> Result<&C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let component_storage = self
            .get_component_storage::<C>()
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))?;

        let component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>(), entity_id))?;

        Ok(&component_storage.component_vec[component_index].1)
    }

    pub fn get_entity_component_mut<C: 'static>(
        &mut self,
        entity_id: EntityId,
    ) -> Result<&mut C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let component_storage = self
            .get_component_storage_mut::<C>()
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))?;

        let component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>(), entity_id))?;

        Ok(&mut component_storage.component_vec[component_index].1)
    }

    pub fn add_entity_component<C: 'static>(
        &mut self,
        entity_id: EntityId,
        component_data: C,
    ) -> Result<(), Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        self.ensure_component_registered::<C>();
        let component_storage = self
            .get_component_storage_mut::<C>()
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))?;

        // Already added?
        if component_storage
            .entity_component_map
            .contains_key(&entity_id)
        {
            return Err(Error::ComponentAlreadyAdded(
                std::any::type_name::<C>(),
                entity_id,
            ));
        }

        let component_index = component_storage.component_vec.len();

        component_storage
            .component_vec
            .push((entity_id, component_data));
        component_storage
            .entity_component_map
            .insert(entity_id, component_index);

        Ok(())
    }

    pub fn remove_entity_component<C: 'static>(&mut self, entity_id: EntityId) -> Result<C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let component_storage = self
            .get_component_storage_mut::<C>()
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))?;

        let entity_component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>(), entity_id))?;

        // Has a different meaning depending on whether it's the entity's component.
        let popped_component = component_storage
            .component_vec
            .pop()
            .expect("There can't be no components, because there is an entity");

        let entity_component_data =
            if entity_component_index == component_storage.component_vec.len() {
                // The last the popped component is what we are looking for
                popped_component.1
            } else {
                // We use the popped component to replace the entity's one.
                component_storage.entity_component_map.remove(&entity_id);

                // Ensure to update the entity component map to the new index
                if let Some(index) = component_storage
                    .entity_component_map
                    .get_mut(&popped_component.0)
                {
                    *index = entity_component_index
                }

                std::mem::replace(
                    &mut component_storage.component_vec[entity_component_index],
                    popped_component,
                )
                .1
            };

        component_storage.entity_component_map.remove(&entity_id);
        Ok(entity_component_data)
    }

    /// Returns `true` if the component was already registered.
    /// Otherwise will register the component.
    pub fn ensure_component_registered<C: 'static>(&mut self) -> bool {
        let component_id = TypeId::of::<C>();
        if self.component_storage_vecs.contains_key(&component_id) {
            true
        } else {
            self.component_storage_vecs
                .insert(component_id, Box::new(ComponentsStorage::<C>::new()));
            false
        }
    }

    fn get_component_storage<C: 'static>(&self) -> Option<&ComponentsStorage<C>> {
        self.component_storage_vecs
            .get(&TypeId::of::<C>())
            .and_then(|cs| (*cs).downcast_ref::<ComponentsStorage<C>>())
    }

    fn get_component_storage_mut<C: 'static>(&mut self) -> Option<&mut ComponentsStorage<C>> {
        self.component_storage_vecs
            .get_mut(&TypeId::of::<C>())
            .and_then(|cs| (*cs).downcast_mut::<ComponentsStorage<C>>())
    }
}

