use std::{
    any::Any,
    collections::{HashMap, hash_map::Keys},
};

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
            self.slots.push(Slot {
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

impl<T> AnyComponentStorage for ComponentStorage<T>
where
    T: 'static,
{
    fn remove(&mut self, entity: Entity) -> bool {
        self.remove(entity).is_some()
    }

    fn entities<'a>(&'a self) -> Keys<'a, Entity, usize> {
        self.entity_to_slot.keys()
    }
}

#[cfg(test)]
mod tests {
    use crate::mem::gen_bit_set::{GenBitSet};

    use super::*;

    fn mock_entities(n: usize) -> Vec<Entity> {
        let mut bitset = GenBitSet::new(n);
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(Entity(bitset.acquire().unwrap()))
        }
        vec
    }

    #[test]
    fn insert() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        // Insertion without deletion so we can inspect directly
        assert_eq!(storage.slots[0].component, "Hello0");
        assert_eq!(storage.slots[0].insert_tick, 0);
        assert_eq!(storage.slots[0].write_tick, None);
        assert_eq!(storage.slots[0].entity, e[0]);

        assert_eq!(storage.slots[1].component, "Hello1");
        assert_eq!(storage.slots[1].insert_tick, 1);
        assert_eq!(storage.slots[1].write_tick, None);
        assert_eq!(storage.slots[1].entity, e[1]);

        assert_eq!(storage.slots[2].component, "Hello2");
        assert_eq!(storage.slots[2].insert_tick, 2);
        assert_eq!(storage.slots[2].write_tick, None);
        assert_eq!(storage.slots[2].entity, e[2]);
    }

    #[test]
    fn insert_twice() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        storage.insert(e[1], "Hello3".into(), 3);

        assert_eq!(storage.slots[1].component, "Hello3");
        assert_eq!(storage.slots[1].insert_tick, 3);
        assert_eq!(storage.slots[1].write_tick, None);
        assert_eq!(storage.slots[1].entity, e[1]);
    }

    #[test]
    fn remove() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        // Remove from the middle: the last slot is swapped into its place
        assert_eq!(storage.remove(e[1]), Some("Hello1".to_string()));

        assert_eq!(storage.slots.len(), 2);
        assert_eq!(storage.entity_to_slot.len(), 2);
        assert!(!storage.entity_to_slot.contains_key(&e[1]));

        assert_eq!(storage.slots[0].entity, e[0]);
        assert_eq!(storage.slots[0].component, "Hello0");
        assert_eq!(storage.slots[1].entity, e[2]);
        assert_eq!(storage.slots[1].component, "Hello2");
        assert_eq!(storage.slots[1].insert_tick, 2);

        // The map must point at the swapped slot's new position
        assert_eq!(storage.entity_to_slot[&e[0]], 0);
        assert_eq!(storage.entity_to_slot[&e[2]], 1);
    }

    #[test]
    fn remove_first() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        assert_eq!(storage.remove(e[0]), Some("Hello0".to_string()));

        assert_eq!(storage.slots.len(), 2);
        assert_eq!(storage.slots[0].entity, e[2]);
        assert_eq!(storage.slots[1].entity, e[1]);
        assert_eq!(storage.entity_to_slot[&e[2]], 0);
        assert_eq!(storage.entity_to_slot[&e[1]], 1);
    }

    #[test]
    fn remove_last() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        // No swap is needed when removing the last slot
        assert_eq!(storage.remove(e[2]), Some("Hello2".to_string()));

        assert_eq!(storage.slots.len(), 2);
        assert_eq!(storage.entity_to_slot.len(), 2);
        assert!(!storage.entity_to_slot.contains_key(&e[2]));
        assert_eq!(storage.entity_to_slot[&e[0]], 0);
        assert_eq!(storage.entity_to_slot[&e[1]], 1);
    }

    #[test]
    fn remove_only_element() {
        let e = mock_entities(1);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);

        assert_eq!(storage.remove(e[0]), Some("Hello0".to_string()));
        assert!(storage.slots.is_empty());
        assert!(storage.entity_to_slot.is_empty());
    }

    #[test]
    fn remove_missing() {
        let e = mock_entities(2);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);

        assert_eq!(storage.remove(e[1]), None);
        assert_eq!(storage.slots.len(), 1);
        assert_eq!(storage.get(e[0]), Some(&"Hello0".to_string()));
    }

    #[test]
    fn get() {
        let e = mock_entities(4);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        assert_eq!(storage.get(e[0]), Some(&"Hello0".to_string()));
        assert_eq!(storage.get(e[1]), Some(&"Hello1".to_string()));
        assert_eq!(storage.get(e[2]), Some(&"Hello2".to_string()));

        // Never inserted
        assert_eq!(storage.get(e[3]), None);

        // `get` must not touch the write tick
        assert_eq!(storage.slots[0].write_tick, None);
    }

    #[test]
    fn get_mut() {
        let e = mock_entities(4);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        storage.get_mut(e[1], 10).unwrap().push_str("_modified");

        assert_eq!(storage.get(e[1]), Some(&"Hello1_modified".to_string()));
        assert_eq!(storage.slots[1].write_tick, Some(10));
        // Insert tick is untouched by writes
        assert_eq!(storage.slots[1].insert_tick, 1);

        // Other slots are untouched
        assert_eq!(storage.slots[0].component, "Hello0");
        assert_eq!(storage.slots[0].write_tick, None);
        assert_eq!(storage.slots[2].component, "Hello2");
        assert_eq!(storage.slots[2].write_tick, None);

        // Never inserted
        assert!(storage.get_mut(e[3], 11).is_none());
    }

    #[test]
    fn get_mut_updates_write_tick() {
        let e = mock_entities(1);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);

        storage.get_mut(e[0], 5);
        assert_eq!(storage.slots[0].write_tick, Some(5));

        storage.get_mut(e[0], 8);
        assert_eq!(storage.slots[0].write_tick, Some(8));
    }

    #[test]
    fn remove_then_get() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        storage.remove(e[1]);

        // Removed entity is gone for both accessors
        assert_eq!(storage.get(e[1]), None);
        assert!(storage.get_mut(e[1], 5).is_none());

        // The survivors (including the swapped one) are still reachable
        assert_eq!(storage.get(e[0]), Some(&"Hello0".to_string()));
        assert_eq!(storage.get(e[2]), Some(&"Hello2".to_string()));
        assert_eq!(
            storage.get_mut(e[2], 6).map(|s| s.clone()),
            Some("Hello2".to_string())
        );
        assert_eq!(storage.slots[1].write_tick, Some(6));
    }

    #[test]
    fn remove_then_remove() {
        let e = mock_entities(3);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        assert_eq!(storage.remove(e[1]), Some("Hello1".to_string()));
        assert_eq!(storage.remove(e[1]), None);

        // The second remove must not have disturbed anything
        assert_eq!(storage.slots.len(), 2);
        assert_eq!(storage.get(e[0]), Some(&"Hello0".to_string()));
        assert_eq!(storage.get(e[2]), Some(&"Hello2".to_string()));
        assert_eq!(storage.entity_to_slot[&e[0]], 0);
        assert_eq!(storage.entity_to_slot[&e[2]], 1);
    }

    #[test]
    fn remove_then_remove_via_trait() {
        let e = mock_entities(2);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);

        assert!(AnyComponentStorage::remove(&mut storage, e[0]));
        assert!(!AnyComponentStorage::remove(&mut storage, e[0]));
        assert_eq!(storage.get(e[1]), Some(&"Hello1".to_string()));
    }

    #[test]
    fn remove_then_insert() {
        let e = mock_entities(4);

        let mut storage = ComponentStorage::<String>::new();
        storage.insert(e[0], "Hello0".into(), 0);
        storage.insert(e[1], "Hello1".into(), 1);
        storage.insert(e[2], "Hello2".into(), 2);

        storage.get_mut(e[1], 3);
        storage.remove(e[1]);

        // Brand-new entity goes at the end
        storage.insert(e[3], "Hello3".into(), 4);
        assert_eq!(storage.slots.len(), 3);
        assert_eq!(storage.slots[2].entity, e[3]);
        assert_eq!(storage.slots[2].component, "Hello3");
        assert_eq!(storage.slots[2].insert_tick, 4);
        assert_eq!(storage.slots[2].write_tick, None);
        assert_eq!(storage.entity_to_slot[&e[3]], 2);

        // Re-inserting the removed entity gets a fresh slot with a fresh write_tick
        storage.insert(e[1], "Hello1_again".into(), 5);
        assert_eq!(storage.slots.len(), 4);
        assert_eq!(storage.slots[3].entity, e[1]);
        assert_eq!(storage.slots[3].component, "Hello1_again");
        assert_eq!(storage.slots[3].insert_tick, 5);
        assert_eq!(storage.slots[3].write_tick, None);
        assert_eq!(storage.entity_to_slot[&e[1]], 3);

        // Everything is still consistent
        assert_eq!(storage.get(e[0]), Some(&"Hello0".to_string()));
        assert_eq!(storage.get(e[2]), Some(&"Hello2".to_string()));
        assert_eq!(storage.get(e[3]), Some(&"Hello3".to_string()));
        assert_eq!(storage.get(e[1]), Some(&"Hello1_again".to_string()));
    }
}
