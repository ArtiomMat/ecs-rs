use std::any::{Any};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ecs::QueryParam;

use super::component_storage::ComponentsStorage;
use super::error::Error;
use super::id_types::{ComponentId, EntityId};

pub struct World {
    /// `dyn Any` is `ComponentStorage<C>`
    pub(super) component_storage_vecs: HashMap<ComponentId, Box<dyn Any>>,
    pub(super) entity_validity_set: HashSet<EntityId>,
    pub(super) entity_counter: AtomicUsize,
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

    /// Get an index of the component in the component storage of `C`
    /// which belongs to this entity.
    fn get_entity_component_index<C: 'static>(&self, entity_id: EntityId) -> Result<usize, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let component_storage = self.get_component_storage::<C>()?;
        component_storage.get_entity_component_index(entity_id)
    }

    /// Get a reference to the component of type `C` that belongs to this
    /// entity.
    // pub fn get_entity_component<C: 'static>(&self, entity_id: EntityId) -> Result<&C, Error> {
    //     let component_index = self.get_entity_component_index::<C>(entity_id)?;

    //     let component_storage = self.get_component_storage::<C>()?;
    //     Ok(&component_storage.components[component_index].1)
    // }

    /// Add a component of type `C` to the entity.
    pub fn add_component<C: 'static>(
        &mut self,
        entity_id: EntityId,
        component: C,
    ) -> Result<(), Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        self.add_component_storage::<C>();

        let component_storage = self.get_component_storage_mut::<C>()?;
        component_storage.add_component(entity_id, component)
    }

    /// Remove the component of type `C` the entity currently has.
    pub fn remove_component<C: 'static>(&mut self, entity_id: EntityId) -> Result<C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let component_storage = self.get_component_storage_mut::<C>()?;
        component_storage.remove_component(entity_id)
    }

    /// Returns `true` if the component was already registered.
    /// Otherwise will register the component.
    pub fn add_component_storage<C: 'static>(&mut self) -> bool {
        let type_id = ComponentId::of::<C>();
        if self.component_storage_vecs.contains_key(&type_id) {
            true
        } else {
            self.component_storage_vecs
                .insert(type_id, Box::new(ComponentsStorage::<C>::new()));
            false
        }
    }

    /// Get a reference to the component storage by the component's type.
    pub(super) fn get_component_storage<C: 'static>(&self) -> Result<&ComponentsStorage<C>, Error> {
        let type_id = &ComponentId::of::<C>();
        self.component_storage_vecs
            .get(&type_id)
            .and_then(|cs| (*cs).downcast_ref::<ComponentsStorage<C>>())
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))
    }

    /// Mutable [`Self::get_component_storage`].
    pub(super) fn get_component_storage_mut<C: 'static>(&mut self) -> Result<&mut ComponentsStorage<C>, Error> {
        let type_id = &ComponentId::of::<C>();
        self.component_storage_vecs
            .get_mut(&type_id)
            .and_then(|cs| (*cs).downcast_mut::<ComponentsStorage<C>>())
            .ok_or(Error::InvalidWorldComponent(std::any::type_name::<C>()))
    }

    fn query_using_iter<'w, T>(&'w self, iter: impl Iterator<Item = &'w EntityId>, mut system: impl FnMut(T::Output)) where T: QueryParam<'w> {
        for &e in iter {
            if T::matches(self, e) {
                system(T::fetch(self, e));
            }
        }
    }

    pub fn query<'w, T>(&'w self, mut system: impl FnMut(T::Output)) where T: QueryParam<'w> {
        for &e in T::guaranteed_matches_iter(self) {
            if T::matches(self, e) {
                system(T::fetch(self, e));
            }
        }
    }
}
