use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Entity {
    /// Indices of the
    indices: HashMap<TypeId, usize>,
}

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct EntityId(usize);

struct ComponentWrapper<T> {
    entity: EntityId,
    data: T,
}

struct State {
    components: HashMap<TypeId, Box<dyn Any>>,
    entities: HashMap<EntityId, Entity>,
    counter: AtomicUsize,
}

#[derive(Debug)]
enum Error<C> {
    InvalidEntityId(EntityId),
    InvalidComponent,

    _FakeUsageOfC(C),
}

impl<C> std::fmt::Display for Error<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidEntityId(entity_id) => write!(f, "Entity {} is invalid", entity_id.0),
            Error::InvalidComponent => {
                write!(f, "Component {} is invalid", std::any::type_name::<C>())
            }
            Error::_FakeUsageOfC(_) => unreachable!(),
        }
    }
}

impl<C: std::fmt::Debug> std::error::Error for Error<C> {}

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

    pub fn get_entity_component<C: 'static>(
        &self,
        entity_id: EntityId,
    ) -> Result<&C, Error<C>> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let entity = self.entities.get(&entity_id).unwrap();
        let component_index = entity
            .indices
            .get(&Self::component_id_for::<C>())
            .ok_or(Error::InvalidComponent)?;
        
        let components = self.get_component_vec::<C>()
            .ok_or(Error::InvalidComponent)?;

        Ok(&components[*component_index].data)
    }

    fn add_component_to_entity<C: 'static>(
        &mut self,
        entity_id: EntityId,
        data: C,
    ) -> Result<(), Error<C>> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        self.ensure_component_registered::<C>();
        let components = self
            .get_component_vec_mut::<C>()
            .expect("The component was supposed to be added");
        components.push(ComponentWrapper {
            entity: entity_id,
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

trait Component {}

#[derive(Debug)]
struct PositionComponent {
    p: [i32; 3],
}

#[derive(Debug)]
struct HealthComponent {
    health: i32,
}

fn main() {
    let mut world = State::new();

    let player_id = world.create_entity();
    world
        .add_component_to_entity(player_id, HealthComponent { health: 100 })
        .unwrap();
    world
        .add_component_to_entity(player_id, PositionComponent { p: [1, 2, 3] })
        .unwrap();

    println!("{}", world.get_entity_component::<HealthComponent>(player_id).unwrap().health);
}
