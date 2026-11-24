use std::{any::Any, collections::{HashMap, hash_map::Keys}};

use crate::ecs::entity::Entity;

pub struct Slot<T> {
    component: T,
    entity: Entity,
    insert_tick: usize,
    write_tick: Option<usize>,
}

pub struct ComponentStorage<T> {
    entity_to_slot: HashMap<Entity, usize>,
    slots: Vec<Slot<T>>,
}

pub trait AnyComponentStorage: Any {
    /// Returns `true` upons successful removal from the entity.
    fn remove(&mut self, entity: Entity) -> bool;

    /// Returns something that you can iterate over for entities.
    fn entities<'a>(&'a self) -> Keys<'a, Entity, usize>;
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            entity_to_slot: HashMap::new(),
            slots: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T, tick: usize) {
        if let Some(&i) = self.entity_to_slot.get(&entity) {
            self.slots[i].insert_tick = tick;
            self.slots[i].component = component;
        } else {
            let i = self.slots.len();
            self.slots.push(Slot{
                insert_tick: tick,
                component: component,
                entity: entity,
                write_tick: None,
            });
            self.entity_to_slot.insert(entity, i);
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let i = *self.entity_to_slot.get(&entity)?;
        Some(&self.slots[i].component)
    }

    pub fn get_mut(&mut self, entity: Entity, tick: usize) -> Option<&mut T> {
        let i = *self.entity_to_slot.get(&entity)?;
        self.slots[i].write_tick = Some(tick);
        Some(&mut self.slots[i].component)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let i = self.entity_to_slot.remove(&entity)?;

        // If the the removed is not last we notify entity_to_slot of the upcoming swap
        let swapped_slot = self.slots.last().expect("entity_to_slot was not empty");
        if swapped_slot.entity != entity {
            self.entity_to_slot.insert(swapped_slot.entity, i);
        }

        let removed_slot = self.slots.swap_remove(i);
        Some(removed_slot.component)
    }
}

impl<T> AnyComponentStorage for ComponentStorage<T> where T: 'static {
    fn remove(&mut self, entity: Entity) -> bool {
        self.remove(entity).is_some()
    }

    fn entities<'a>(&'a self) -> Keys<'a, Entity, usize> {
        self.entity_to_slot.keys()
    }
}
