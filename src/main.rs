use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Entity {
    /// Indices of the
    component_indices: HashMap<TypeId, usize>,
}

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct EntityId(usize);

struct ComponentDataWrapper<T> {
    entity_id: EntityId,
    data: T,
}

struct World {
    component_vecs: HashMap<TypeId, Box<dyn Any>>,
    entities: HashMap<EntityId, Entity>,
    entity_counter: AtomicUsize,
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

impl World {
    pub fn new() -> Self {
        Self {
            component_vecs: HashMap::new(),
            entities: HashMap::new(),
            entity_counter: 0.into(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = EntityId(self.entity_counter.fetch_add(1, Ordering::Relaxed));
        let entity = Entity {
            component_indices: HashMap::new(),
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
            .component_indices
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
            .component_indices
            .get(&Self::component_id_for::<C>())
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let component_vec = self
            .get_component_vec_mut::<C>()
            .ok_or(Self::new_invalid_component_err::<C>())?;

        Ok(&mut component_vec[component_index].data)
    }

    fn new_invalid_component_err<C: 'static>() -> Error {
        Error::InvalidComponent(std::any::type_name::<C>())
    }

    fn add_entity_component<C: 'static>(
        &mut self,
        entity_id: EntityId,
        component_data: C,
    ) -> Result<(), Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        self.ensure_component_registered::<C>();
        let component_vec = self
            .get_component_vec_mut::<C>()
            .expect("The component was supposed to be added");
        component_vec.push(ComponentDataWrapper {
            entity_id,
            data: component_data,
        });
        let component_vec_len = component_vec.len();

        // unwrap because we already checked in start.
        let entity = self.entities.get_mut(&entity_id).unwrap();
        entity
            .component_indices
            .insert(Self::component_id_for::<C>(), component_vec_len - 1);

        Ok(())
    }

    fn remove_entity_component<C: 'static>(&mut self, entity_id: EntityId) -> Result<C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let entity = self.entities.get_mut(&entity_id).unwrap();
        let component_index = entity
            .component_indices
            .remove(&Self::component_id_for::<C>())
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let component_vec = self
            .get_component_vec_mut::<C>()
            .ok_or(Self::new_invalid_component_err::<C>())?;
        let mut last_component = component_vec
            .pop()
            .expect("There can't be no components, because there is an entity");

        // If we already popped the entity as last there is no
        // need to do anything else, otherwise below is some
        // code below that replaces the entity's component with this
        // popped one.
        if last_component.entity_id != entity_id {
            std::mem::swap(&mut component_vec[component_index], &mut last_component);
            // TODO: Update the swapped entity.
        }

        Ok(last_component.data)
    }
    fn component_id_for<C: 'static>() -> TypeId {
        TypeId::of::<Box<Vec<ComponentDataWrapper<C>>>>()
    }

    fn ensure_component_registered<C: 'static>(&mut self) -> bool {
        let component_id = Self::component_id_for::<C>();
        if self.component_vecs.contains_key(&component_id) {
            true
        } else {
            self.component_vecs
                .insert(component_id, Box::new(Vec::<ComponentDataWrapper<C>>::new()));
            false
        }
    }

    fn get_component_vec<C: 'static>(&self) -> Option<&Vec<ComponentDataWrapper<C>>> {
        self.component_vecs
            .get(&Self::component_id_for::<C>())
            .and_then(|c| c.downcast_ref::<Vec<ComponentDataWrapper<C>>>())
    }

    fn get_component_vec_mut<C: 'static>(&mut self) -> Option<&mut Vec<ComponentDataWrapper<C>>> {
        self.component_vecs
            .get_mut(&Self::component_id_for::<C>())
            .and_then(|c| c.downcast_mut::<Vec<ComponentDataWrapper<C>>>())
    }
}

struct PositionComponent {
    p: [i32; 3],
}

struct HealthComponent {
    health: i32,
}

struct PlayerTag;

// src/lib.rs (tests section)
#[cfg(test)] // Only compile when running tests
mod tests {
    use super::*; // Import items from the parent module

    #[test] // Marks this function as a test
    fn single_entity_component_sanity() {
        let mut world = World::new();

        let player_id = world.create_entity();
        world
            .add_entity_component(player_id, HealthComponent { health: 100 })
            .unwrap();
        world
            .add_entity_component(player_id, PositionComponent { p: [1, 2, 3] })
            .unwrap();
        world
            .add_entity_component(player_id, PlayerTag)
            .unwrap();

        assert_eq!(
            100,
            world
                .get_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .health
        );

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

fn main() {
    
}
