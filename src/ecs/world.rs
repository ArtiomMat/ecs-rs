use std::{
    any::{Any, TypeId}, cell::{Ref, RefCell}, collections::HashMap,
};

use crate::{
    ecs::{
        entity::Entity, storages::{ComponentStorage, ComponentStorageTrait}, system_params::{system_param::SystemParam, world_ptr::UnsafeWorldCell},
    }, mem::gen_bit_set::GenBitSet,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Already despawned {0:?}")]
    Despawned(Entity),
    #[error("Component was not found on {0:?}")]
    ComponentNotFound(Entity),
}

pub struct World {
    tick_count: usize,
    entities_set: GenBitSet,
    component_storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_set: GenBitSet::new(8),
            component_storages: HashMap::new(),
            tick_count: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let gen_bit = self
            .entities_set
            .acquire()
            .or_else(|| {
                self.entities_set.resize(self.entities_set.len() * 2);
                self.entities_set.acquire()
            })
            .unwrap();

        Entity(gen_bit)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), Error> {
        self.entities_set
            .release(*entity)
            .map_err(|_| Error::Despawned(entity))
    }

    fn get_component_storage_mut<T: 'static>(&mut self) -> &mut ComponentStorage<T> {
        let type_id = TypeId::of::<T>();

        let type_erased = self
            .component_storages
            .entry(type_id)
            .or_insert(Box::new(ComponentStorage::<T>::new()));

        type_erased.downcast_mut::<ComponentStorage<T>>().unwrap()
    }

    fn get_component_storage<T: 'static>(&self) -> Option<&ComponentStorage<T>> {
        let type_id = TypeId::of::<T>();

        let type_erased = self.component_storages.get(&type_id)?;

        type_erased.downcast_ref()
    }

    pub fn insert_component<T: 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), Error> {
        if !self.entities_set.contains(*entity) {
            return Err(Error::Despawned(entity));
        }

        let tick = self.tick_count;

        let storage = self.get_component_storage_mut::<T>();
        storage.insert(entity, component, tick);

        Ok(())
    }

    pub fn get_component<T: 'static>(
        &self,
        entity: Entity,
    ) -> Result<&T, Error> {
        if !self.entities_set.contains(*entity) {
            return Err(Error::Despawned(entity));
        }

        let storage = self.get_component_storage::<T>().ok_or(Error::ComponentNotFound(entity))?;

        storage.get(entity).ok_or(Error::ComponentNotFound(entity))
    }

    pub fn run_system<'w, F, A>(&'w mut self, sys: F, entity: Entity) where F: FnOnce(A::Item<'w>), A: SystemParam {
        {
            let cell = unsafe { UnsafeWorldCell::new(self) };
            sys(A::get(cell, entity))
        }
    }
}
