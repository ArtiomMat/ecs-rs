use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
};

use crate::{
    ecs::{
        entity::Entity,
        storages::{AnyComponentStorage, ComponentStorage},
        system_params::{system_param::SystemParam, world_ptr::WorldPtr},
    },
    mem::gen_bit_set::GenBitSet,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Already despawned {0:?}")]
    Despawned(Entity),
    #[error("Component was not found on {0:?}")]
    ComponentNotFound(Entity),
}

/// All spawned entities have this component.
///
/// It is a crucial part of cleanup of despawned entities.
/// Without it the world cannot remove components.
pub(super) struct ValidEntityComponent {
    owned_components: HashSet<TypeId>,
}

pub struct World {
    tick_count: usize,
    entities_set: GenBitSet,
    component_storages: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
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

        let entity = Entity(gen_bit);

        self.insert_component(
            entity,
            ValidEntityComponent {
                owned_components: HashSet::new(),
            },
        )
        .expect("Entity already exists");

        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), Error> {
        self.entities_set
            .release(*entity)
            .map_err(|_| Error::Despawned(entity))?;

        let valid_component = self
            .remove_component::<ValidEntityComponent>(entity)
            .expect("Exists in entities_set");

        for type_id in valid_component.owned_components.iter() {
            let storage = self
                .component_storages
                .get_mut(type_id)
                .expect("Registered as owned");
            storage.remove(entity);
        }

        Ok(())
    }

    /// If the component storage doesn't already exist creates it. So can't error.
    ///
    /// Returns a reference to the component storage of components `T`.
    pub(super) fn get_component_storage_mut<T: 'static>(&mut self) -> &mut ComponentStorage<T> {
        let type_id = TypeId::of::<T>();

        let type_erased: &mut dyn Any = &mut **self
            .component_storages
            .entry(type_id)
            .or_insert(Box::new(ComponentStorage::<T>::new()));

        type_erased.downcast_mut::<ComponentStorage<T>>().unwrap()
    }

    /// Returns a reference to the component storage of components `T`.
    /// May return `None` if the component storage of components `T` doesn't exist.
    pub(super) fn get_component_storage<T: 'static>(&self) -> Option<&ComponentStorage<T>> {
        let type_id = TypeId::of::<T>();

        let type_erased: &dyn Any = &**self.component_storages.get(&type_id)?;

        type_erased.downcast_ref()
    }

    /// Returns a reference to the component storage of components `T`.
    /// The form however of the return value is `AnyComponentStorage` which
    /// sidesteps generics at the cost of less freedom.
    pub(super) fn get_any_component_storage(
        &self,
        type_id: TypeId,
    ) -> Option<&dyn AnyComponentStorage> {
        let type_erased: &dyn AnyComponentStorage = &**self.component_storages.get(&type_id)?;
        Some(type_erased)
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

        let valid_storage = self.get_component_storage_mut::<ValidEntityComponent>();
        let valid_component = valid_storage
            .get_mut(entity, tick)
            .expect("All entites have this component");
        valid_component.owned_components.insert(TypeId::of::<T>());

        Ok(())
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Result<&T, Error> {
        if !self.entities_set.contains(*entity) {
            return Err(Error::Despawned(entity));
        }

        let storage = self
            .get_component_storage::<T>()
            .ok_or(Error::ComponentNotFound(entity))?;

        storage.get(entity).ok_or(Error::ComponentNotFound(entity))
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Result<T, Error> {
        if !self.entities_set.contains(*entity) {
            return Err(Error::Despawned(entity));
        }

        let tick = self.tick_count;

        let valid_storage = self.get_component_storage_mut::<ValidEntityComponent>();
        let valid_component = valid_storage
            .get_mut(entity, tick)
            .expect("All entites have this component");
        valid_component.owned_components.remove(&TypeId::of::<T>());

        let storage = self.get_component_storage_mut::<T>();

        storage
            .remove(entity)
            .ok_or(Error::ComponentNotFound(entity))
    }

    /// Returns a mutable reference to the component.
    ///
    /// Considers it dirty since this tick.
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Result<&mut T, Error> {
        if !self.entities_set.contains(*entity) {
            return Err(Error::Despawned(entity));
        }

        let tick = self.tick_count;

        let storage = self.get_component_storage_mut::<T>();

        storage
            .get_mut(entity, tick)
            .ok_or(Error::ComponentNotFound(entity))
    }

    pub fn run_system<'w, F, A>(&'w mut self, sys: F, entity: Entity)
    where
        F: FnOnce(A::Item<'w>),
        A: SystemParam,
    {
        {
            let cell = WorldPtr::new(self);
            sys(A::fetch(cell))
        }
    }
}
