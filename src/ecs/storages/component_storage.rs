use std::{any::Any, collections::HashMap};

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

pub trait ComponentStorageTrait: Any {}

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

    pub fn get_mut(&mut self, entity: Entity, tick: usize) -> Option<&T> {
        let i = *self.entity_to_slot.get(&entity)?;
        self.slots[i].write_tick = Some(tick);
        Some(&mut self.slots[i].component)
    }
}
