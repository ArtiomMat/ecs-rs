use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Entity {
    /// Indices of the
    indices: HashMap<TypeId, usize>,
}

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct EntityId(usize);

struct ComponentWrapper<T> {
    entity_id: EntityId,
    data: T,
}

struct State {
    components: HashMap<TypeId, Box<dyn Any>>,
    entities: HashMap<EntityId, Entity>,
    counter: AtomicUsize,
}

#[derive(Debug)]
enum Error {
    InvalidEntityId(EntityId),
    InvalidComponent(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidEntityId(entity_id) => write!(f, "Entity {} is invalid", entity_id.0),
            Error::InvalidComponent(name) => {
                write!(f, "Component {} is invalid", name)
            }
        }
    }
}

impl std::error::Error for Error {}

impl State {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            entities: HashMap::new(),
            counter: 0.into(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = EntityId(self.counter.fetch_add(1, Ordering::Relaxed));
        let entity = Entity {
            indices: HashMap::new(),
        };

        self.entities.insert(entity_id, entity);

        entity_id
    }

    pub fn is_entity_valid(&self, id: EntityId) -> bool {
        self.entities.contains_key(&id)
    }

    pub fn get_entity_component<C: 'static>(&self, entity_id: EntityId) -> Result<&C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let entity = self.entities.get(&entity_id).unwrap();
        let component_index = *entity
            .indices
            .get(&Self::component_id_for::<C>())
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let components = self
            .get_component_vec::<C>()
            .ok_or(Self::new_invalid_component_err::<C>())?;

        Ok(&components[component_index].data)
    }

    pub fn get_entity_component_mut<C: 'static>(
        &mut self,
        entity_id: EntityId,
    ) -> Result<&mut C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let entity = self.entities.get(&entity_id).unwrap();
        let component_index = *entity
            .indices
            .get(&Self::component_id_for::<C>())
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let components = self
            .get_component_vec_mut::<C>()
            .ok_or(Self::new_invalid_component_err::<C>())?;

        Ok(&mut components[component_index].data)
    }

    fn new_invalid_component_err<C: 'static>() -> Error {
        Error::InvalidComponent(std::any::type_name::<C>())
    }

    fn add_entity_component<C: 'static>(
        &mut self,
        entity_id: EntityId,
        data: C,
    ) -> Result<(), Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        self.ensure_component_registered::<C>();
        let components = self
            .get_component_vec_mut::<C>()
            .expect("The component was supposed to be added");
        components.push(ComponentWrapper {
            entity_id,
            data: data,
        });
        let new_components_len = components.len();

        // unwrap because we already checked in start.
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity
            .indices
            .insert(Self::component_id_for::<C>(), new_components_len - 1);

        Ok(())
    }

    fn remove_entity_component<C: 'static>(&mut self, entity_id: EntityId) -> Result<C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let entity = self.entities.get_mut(&entity_id).unwrap();
        let component_index = entity
            .indices
            .remove(&Self::component_id_for::<C>())
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let components = self
            .get_component_vec_mut::<C>()
            .ok_or(Self::new_invalid_component_err::<C>())?;
        let mut last_component = components
            .pop()
            .expect("There can't be no components, because there is an entity");

        // If we already popped the entity as last there is no
        // need to do anything else, otherwise below is some
        // code below that replaces the entity's component with this
        // popped one.
        if last_component.entity_id != entity_id {
            std::mem::swap(&mut components[component_index], &mut last_component);
        }

        Ok(last_component.data)
    }
    fn component_id_for<C: 'static>() -> TypeId {
        TypeId::of::<Box<Vec<ComponentWrapper<C>>>>()
    }

    fn ensure_component_registered<C: 'static>(&mut self) -> bool {
        let component_id = Self::component_id_for::<C>();
        if self.components.contains_key(&component_id) {
            true
        } else {
            self.components
                .insert(component_id, Box::new(Vec::<ComponentWrapper<C>>::new()));
            false
        }
    }

    fn get_component_vec<C: 'static>(&self) -> Option<&Vec<ComponentWrapper<C>>> {
        self.components
            .get(&Self::component_id_for::<C>())
            .and_then(|components| components.downcast_ref::<Vec<ComponentWrapper<C>>>())
    }

    fn get_component_vec_mut<C: 'static>(&mut self) -> Option<&mut Vec<ComponentWrapper<C>>> {
        self.components
            .get_mut(&Self::component_id_for::<C>())
            .and_then(|components| components.downcast_mut::<Vec<ComponentWrapper<C>>>())
    }
}

struct PositionComponent {
    p: [i32; 3],
}

struct HealthComponent {
    health: i32,
}

// src/lib.rs (tests section)
#[cfg(test)] // Only compile when running tests
mod tests {
    use super::*; // Import items from the parent module

    #[test] // Marks this function as a test
    fn single_entity_component_sanity() {
        let mut world = State::new();

        let player_id = world.create_entity();
        world
            .add_entity_component(player_id, HealthComponent { health: 100 })
            .unwrap();

        assert_eq!(
            100,
            world
                .get_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .health
        );

        world
            .add_entity_component(player_id, PositionComponent { p: [1, 2, 3] })
            .unwrap();

        world
            .get_entity_component_mut::<HealthComponent>(player_id)
            .unwrap()
            .health = 67;

        assert_eq!(
            67,
            world
                .get_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .health
        );

        assert_eq!(
            67,
            world
                .remove_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .health
        );

        assert!(
            world
                .remove_entity_component::<HealthComponent>(player_id)
                .is_err()
        );

        println!(
            "{}",
            world
                .remove_entity_component::<HealthComponent>(player_id)
                .err()
                .unwrap()
        );

        assert!(
            world
                .get_entity_component::<HealthComponent>(player_id)
                .is_err()
        );
        assert_eq!(
            [1, 2, 3],
            world
                .get_entity_component::<PositionComponent>(player_id)
                .unwrap()
                .p
        );
    }
}

fn main() {}
