use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

// struct Entity {
//     /// Indices of the
//     component_indices: HashMap<TypeId, usize>,
// }

/// Comparing it can be useful sometimes:
///
/// - `a > b` means that `a` was allocated after `b`.
/// - `a == b` means that `a` refers to the same underlying entity as `b`.
///
/// Non-comarison traits are mostly derived for internal use, but are there for
/// your use too.
#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct EntityId(usize);

struct ComponentsStorage<C: 'static> {
    component_vec: Vec<(EntityId, C)>,
    /// A map between entity IDs and their respective component index
    entity_component_map: HashMap<EntityId, usize>,
}

impl<C> ComponentsStorage<C> {
    fn new() -> Self {
        Self {
            component_vec: Vec::new(),
            entity_component_map: HashMap::new(),
        }
    }
}

struct World {
    component_storage_vecs: HashMap<TypeId, Box<dyn Any>>,
    // component_vecs: HashMap<TypeId, Box<dyn Any>>,
    // entities: HashMap<EntityId, Entity>,
    entity_validity_set: HashSet<EntityId>,
    entity_counter: AtomicUsize,
}

#[derive(Debug)]
enum Error {
    InvalidEntityId(EntityId),
    InvalidWorldComponent(&'static str),
    InvalidEntityComponent(&'static str),
    ComponentAlreadyAdded(&'static str, EntityId),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidEntityId(entity_id) => write!(f, "Entity {} is invalid", entity_id.0),
            Error::InvalidWorldComponent(name) => {
                write!(
                    f,
                    "Component {} was never registered to any entity in the world",
                    name
                )
            }
            Error::InvalidEntityComponent(name) => {
                write!(f, "Component {} was never registered to the entity", name)
            }
            Error::ComponentAlreadyAdded(name, entity_id) => {
                write!(
                    f,
                    "Component {} was already added to entity {}",
                    name, entity_id.0
                )
            }
        }
    }
}

impl std::error::Error for Error {}

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
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>()))?;

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
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>()))?;

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
            .ok_or(Self::new_invalid_component_err::<C>())?;

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
            .ok_or(Self::new_invalid_component_err::<C>())?;

        let entity_component_index = *component_storage
            .entity_component_map
            .get(&entity_id)
            .ok_or(Error::InvalidEntityComponent(std::any::type_name::<C>()))?;

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

    fn new_invalid_component_err<C: 'static>() -> Error {
        Error::InvalidWorldComponent(std::any::type_name::<C>())
    }

    fn component_id_for<C: 'static>() -> TypeId {
        TypeId::of::<Box<Vec<C>>>()
    }

    fn ensure_component_registered<C: 'static>(&mut self) -> bool {
        let component_id = Self::component_id_for::<C>();
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
            .get(&Self::component_id_for::<C>())
            .and_then(|cs| (*cs).downcast_ref::<ComponentsStorage<C>>())
    }

    fn get_component_storage_mut<C: 'static>(&mut self) -> Option<&mut ComponentsStorage<C>> {
        self.component_storage_vecs
            .get_mut(&Self::component_id_for::<C>())
            .and_then(|cs| (*cs).downcast_mut::<ComponentsStorage<C>>())
    }
}

// src/lib.rs (tests section)
#[cfg(test)] // Only compile when running tests
mod tests {
    use super::*; // Import items from the parent module

    struct PositionComponent([i32; 3]);

    struct HealthComponent(i32);

    struct PlayerTag;
    /// Ensures entities don't get mixed when removing and adding components.
    #[test]
    fn multiple_entities_not_mixed() {
        let mut world = World::new();

        let a = world.create_entity();
        let b = world.create_entity();
        let c = world.create_entity();

        world
            .add_entity_component(c, PositionComponent([2, 2, 2]))
            .unwrap();
        world
            .add_entity_component(a, PositionComponent([0, 0, 0]))
            .unwrap();
        world
            .add_entity_component(b, PositionComponent([1, 1, 1]))
            .unwrap();

        world.add_entity_component(b, PlayerTag).unwrap();
        world.add_entity_component(a, PlayerTag).unwrap();

        world.add_entity_component(b, HealthComponent(1)).unwrap();
        world.add_entity_component(c, HealthComponent(2)).unwrap();
        world.add_entity_component(a, HealthComponent(0)).unwrap();

        // First let's remove the last component that was added in HealtComponent
        assert_eq!(
            0,
            world
                .remove_entity_component::<HealthComponent>(a)
                .unwrap()
                .0
        );
        // Then remove the first that was added.
        assert_eq!(
            1,
            world
                .remove_entity_component::<HealthComponent>(b)
                .unwrap()
                .0
        );
        // Then remove the last remaining.
        assert_eq!(
            2,
            world
                .remove_entity_component::<HealthComponent>(c)
                .unwrap()
                .0
        );

        // First let's remove the first component that was added in HealtComponent
        assert_eq!(
            [2, 2, 2],
            world
                .remove_entity_component::<PositionComponent>(c)
                .unwrap()
                .0
        );
        // Then the second that was added which is now the first.
        assert_eq!(
            [0, 0, 0],
            world
                .remove_entity_component::<PositionComponent>(a)
                .unwrap()
                .0
        );
        // Then remove the last remaining.
        assert_eq!(
            [1, 1, 1],
            world
                .remove_entity_component::<PositionComponent>(b)
                .unwrap()
                .0
        );
    }

    #[test]
    fn single_entity_component_sanity() {
        let mut world = World::new();

        let player_id = world.create_entity();
        world
            .add_entity_component(player_id, HealthComponent(100))
            .unwrap();
        world
            .add_entity_component(player_id, PositionComponent([1, 2, 3]))
            .unwrap();
        world.add_entity_component(player_id, PlayerTag).unwrap();

        assert!(
            world.add_entity_component(player_id, PlayerTag).is_err(),
            "Adding again is an error."
        );

        assert_eq!(
            100,
            world
                .get_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .0
        );

        world
            .get_entity_component_mut::<HealthComponent>(player_id)
            .unwrap()
            .0 = 67;

        assert_eq!(
            67,
            world
                .get_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .0
        );

        assert_eq!(
            67,
            world
                .remove_entity_component::<HealthComponent>(player_id)
                .unwrap()
                .0
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
                .0
        );
    }
}

fn main() {}
