use std::any::{Any, TypeId, type_name};
use std::collections::{HashMap, HashSet};

use super::component_storage::ComponentsStorage;
use super::entity_id::EntityId;
use super::error::Error;

pub struct World {
    /// `dyn Any` is a [`ComponentsStorage`]
    pub(super) component_storage_vecs: HashMap<TypeId, Box<dyn Any>>,
    pub(super) entity_validity_set: HashSet<EntityId>,
    pub(super) entity_counter: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_storage_vecs: HashMap::new(),
            entity_validity_set: HashSet::new(),
            entity_counter: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = EntityId(self.entity_counter);
        assert!(
            !self.entity_validity_set.contains(&entity_id),
            "We wrapped around and {} is still valid",
            entity_id
        );
        self.entity_counter = self.entity_counter.wrapping_add(1);

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

        let Some(component_storage) = self.get_component_storage::<C>() else {
            return Err(Error::InvalidWorldComponent(type_name::<C>()));
        };
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

        let component_storage = self.get_component_storage_mut::<C>().unwrap();
        component_storage.add_component(entity_id, component)
    }

    /// Remove the component of type `C` the entity currently has.
    pub fn remove_component<C: 'static>(&mut self, entity_id: EntityId) -> Result<C, Error> {
        if !self.is_entity_valid(entity_id) {
            return Err(Error::InvalidEntityId(entity_id));
        }

        let Some(component_storage) = self.get_component_storage_mut::<C>() else {
            return Err(Error::InvalidWorldComponent(type_name::<C>()));
        };
        component_storage.remove_component(entity_id)
    }

    /// Returns `true` if the component was already registered.
    /// Otherwise will register the component.
    pub fn add_component_storage<C: 'static>(&mut self) -> bool {
        let type_id = TypeId::of::<C>();
        if self.component_storage_vecs.contains_key(&type_id) {
            true
        } else {
            self.component_storage_vecs
                .insert(type_id, Box::new(ComponentsStorage::<C>::new()));
            false
        }
    }

    /// Get a reference to the component storage by the component's type.
    pub(super) fn get_component_storage<C: 'static>(&self) -> Option<&ComponentsStorage<C>> {
        let type_id = &TypeId::of::<C>();
        self.component_storage_vecs
            .get(&type_id)
            .and_then(|cs| (*cs).downcast_ref::<ComponentsStorage<C>>())
    }

    /// Mutable [`Self::get_component_storage`].
    pub(super) fn get_component_storage_mut<C: 'static>(
        &mut self,
    ) -> Option<&mut ComponentsStorage<C>> {
        let type_id = &TypeId::of::<C>();
        self.component_storage_vecs
            .get_mut(&type_id)
            .and_then(|cs| (*cs).downcast_mut::<ComponentsStorage<C>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helper component types -------------------------------------

    #[derive(Debug, PartialEq, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Name(String);

    // ---- entity creation / validity -----------------------------------

    #[test]
    fn new_world_has_no_entities() {
        let world = World::new();
        assert!(world.component_storage_vecs.is_empty());
        assert!(world.entity_validity_set.is_empty());
    }

    #[test]
    fn create_entity_returns_valid_entity() {
        let mut world = World::new();
        let entity = world.create_entity();
        assert!(world.is_entity_valid(entity));
    }

    #[test]
    fn create_entity_ids_are_unique_and_increasing() {
        let mut world = World::new();
        let e0 = world.create_entity();
        let e1 = world.create_entity();
        let e2 = world.create_entity();

        assert_ne!(e0, e1);
        assert_ne!(e1, e2);
        assert_ne!(e0, e2);

        // EntityId wraps a usize counter starting at 0 and incrementing.
        assert_eq!(e0, EntityId(0));
        assert_eq!(e1, EntityId(1));
        assert_eq!(e2, EntityId(2));
    }

    #[test]
    fn unknown_entity_is_not_valid() {
        let world = World::new();
        assert!(!world.is_entity_valid(EntityId(999)));
    }

    #[test]
    fn entity_from_other_world_is_not_valid_here() {
        let mut world_a = World::new();
        let world_b = World::new();

        let entity = world_a.create_entity();
        assert!(!world_b.is_entity_valid(entity));
    }

    // ---- add_component_storage -----------------------------------------

    #[test]
    fn add_component_storage_registers_new_type_once() {
        let mut world = World::new();

        // First registration: type was not previously registered -> false.
        let already_registered = world.add_component_storage::<Position>();
        assert!(!already_registered);
        assert!(world.get_component_storage::<Position>().is_some());

        // Second registration of the same type: already present -> true.
        let already_registered_again = world.add_component_storage::<Position>();
        assert!(already_registered_again);
    }

    #[test]
    fn add_component_storage_is_independent_per_type() {
        let mut world = World::new();
        world.add_component_storage::<Position>();
        world.add_component_storage::<Velocity>();

        assert!(world.get_component_storage::<Position>().is_some());
        assert!(world.get_component_storage::<Velocity>().is_some());
        // Name was never registered.
        assert!(world.get_component_storage::<Name>().is_none());
    }

    // ---- get_component_storage / get_component_storage_mut ------------

    #[test]
    fn get_component_storage_errors_when_unregistered() {
        let world = World::new();
        let result = world.get_component_storage::<Position>();
        assert!(result.is_none());
    }

    #[test]
    fn get_component_storage_mut_errors_when_unregistered() {
        let mut world = World::new();
        let result = world.get_component_storage_mut::<Position>();
        assert!(result.is_none());
    }

    // ---- add_component --------------------------------------------------

    #[test]
    fn add_component_succeeds_for_valid_entity() {
        let mut world = World::new();
        let entity = world.create_entity();

        let result = world.add_component(entity, Position { x: 1.0, y: 2.0 });
        assert!(result.is_ok());
    }

    #[test]
    fn add_component_auto_registers_storage() {
        let mut world = World::new();
        let entity = world.create_entity();

        // No explicit add_component_storage call beforehand.
        assert!(world.get_component_storage::<Position>().is_none());

        world
            .add_component(entity, Position { x: 0.0, y: 0.0 })
            .unwrap();

        assert!(world.get_component_storage::<Position>().is_some());
    }

    #[test]
    fn add_component_fails_for_invalid_entity() {
        let mut world = World::new();
        let bogus = EntityId(42);

        let result = world.add_component(bogus, Position { x: 1.0, y: 1.0 });
        assert!(matches!(result, Err(Error::InvalidEntityId(id)) if id == bogus));
    }

    #[test]
    fn add_multiple_component_types_to_same_entity() {
        let mut world = World::new();
        let entity = world.create_entity();

        world
            .add_component(entity, Position { x: 1.0, y: 2.0 })
            .unwrap();
        world
            .add_component(entity, Velocity { dx: 0.5, dy: -0.5 })
            .unwrap();
        world
            .add_component(entity, Name("Player".to_string()))
            .unwrap();

        assert!(world.get_component_storage::<Position>().is_some());
        assert!(world.get_component_storage::<Velocity>().is_some());
        assert!(world.get_component_storage::<Name>().is_some());
    }

    #[test]
    fn add_component_same_type_to_different_entities() {
        let mut world = World::new();
        let e1 = world.create_entity();
        let e2 = world.create_entity();

        world
            .add_component(e1, Position { x: 1.0, y: 1.0 })
            .unwrap();
        world
            .add_component(e2, Position { x: 2.0, y: 2.0 })
            .unwrap();

        // Both entities should have distinct component storage slots.
        assert!(world.get_entity_component_index::<Position>(e1).is_ok());
        assert!(world.get_entity_component_index::<Position>(e2).is_ok());
    }

    // ---- remove_component ------------------------------------------------

    #[test]
    fn remove_component_returns_the_component_value() {
        let mut world = World::new();
        let entity = world.create_entity();
        let pos = Position { x: 3.0, y: 4.0 };

        world.add_component(entity, pos.clone()).unwrap();
        let removed = world.remove_component::<Position>(entity).unwrap();

        assert_eq!(removed, pos);
    }

    #[test]
    fn remove_component_fails_for_invalid_entity() {
        let mut world = World::new();
        world.add_component_storage::<Position>();
        let bogus = EntityId(123);

        let result = world.remove_component::<Position>(bogus);
        assert!(matches!(result, Err(Error::InvalidEntityId(id)) if id == bogus));
    }

    #[test]
    fn remove_component_fails_when_storage_never_registered() {
        let mut world = World::new();
        let entity = world.create_entity();

        // Position storage was never registered via add_component_storage
        // or add_component.
        let result = world.remove_component::<Position>(entity);
        assert!(matches!(result, Err(Error::InvalidWorldComponent(_))));
    }

    #[test]
    fn remove_component_fails_when_entity_never_had_it() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_component_storage::<Position>();

        // Storage is registered, but this entity never got a Position.
        let result = world.remove_component::<Position>(entity);
        assert!(result.is_err());
    }

    #[test]
    fn remove_component_twice_fails_second_time() {
        let mut world = World::new();
        let entity = world.create_entity();
        world
            .add_component(entity, Position { x: 0.0, y: 0.0 })
            .unwrap();

        assert!(world.remove_component::<Position>(entity).is_ok());
        assert!(world.remove_component::<Position>(entity).is_err());
    }

    #[test]
    fn remove_component_does_not_affect_other_entities() {
        let mut world = World::new();
        let e1 = world.create_entity();
        let e2 = world.create_entity();

        world
            .add_component(e1, Position { x: 1.0, y: 1.0 })
            .unwrap();
        world
            .add_component(e2, Position { x: 2.0, y: 2.0 })
            .unwrap();

        let removed = world.remove_component::<Position>(e1).unwrap();
        assert_eq!(removed, Position { x: 1.0, y: 1.0 });

        // e2's component should still be retrievable/removable.
        let still_there = world.remove_component::<Position>(e2).unwrap();
        assert_eq!(still_there, Position { x: 2.0, y: 2.0 });
    }

    #[test]
    fn remove_component_does_not_affect_other_component_types() {
        let mut world = World::new();
        let entity = world.create_entity();

        world
            .add_component(entity, Position { x: 1.0, y: 1.0 })
            .unwrap();
        world
            .add_component(entity, Velocity { dx: 0.1, dy: 0.2 })
            .unwrap();

        world.remove_component::<Position>(entity).unwrap();

        // Velocity should be unaffected and still removable.
        let vel = world.remove_component::<Velocity>(entity).unwrap();
        assert_eq!(vel, Velocity { dx: 0.1, dy: 0.2 });
    }

    // ---- swap_remove correctness -----------------------------------------
    //
    // If ComponentsStorage::remove_component uses Vec::swap_remove, the last
    // element moves into the removed slot. A correct implementation must
    // update that moved entity's index bookkeeping. These tests specifically
    // target that bookkeeping by removing from the *middle* or *front* of a
    // multi-entity storage and then proving the *last* entity (the one that
    // gets swapped) still resolves to its own, correct component -- not the
    // stale one, not out-of-bounds, and not silently the removed one's slot.

    #[test]
    fn remove_middle_entity_preserves_last_entity_value_via_swap_remove() {
        let mut world = World::new();
        let e0 = world.create_entity();
        let e1 = world.create_entity();
        let e2 = world.create_entity(); // will be the "last" element in storage

        world
            .add_component(e0, Position { x: 0.0, y: 0.0 })
            .unwrap();
        world
            .add_component(e1, Position { x: 1.0, y: 1.0 })
            .unwrap();
        world
            .add_component(e2, Position { x: 2.0, y: 2.0 })
            .unwrap();

        // Remove the middle entry. Under swap_remove(1), e2's component
        // (the last element) gets physically moved into index 1.
        let removed = world.remove_component::<Position>(e1).unwrap();
        assert_eq!(removed, Position { x: 1.0, y: 1.0 });

        // e0 must be completely untouched.
        let idx0 = world.get_entity_component_index::<Position>(e0);
        assert!(idx0.is_ok());

        // e2's component must still resolve to e2's OWN value (2.0, 2.0),
        // not e1's stale value, and its index lookup must not panic/OOB.
        let idx2 = world.get_entity_component_index::<Position>(e2);
        assert!(
            idx2.is_ok(),
            "e2 should still have a valid component index after swap"
        );

        let e2_value = world.remove_component::<Position>(e2).unwrap();
        assert_eq!(
            e2_value,
            Position { x: 2.0, y: 2.0 },
            "swap_remove must update the moved entity's index bookkeeping; \
             got wrong value, meaning e2's index still points at stale/removed data"
        );

        // e0 should still be fully intact and correct after everything else moved.
        let e0_value = world.remove_component::<Position>(e0).unwrap();
        assert_eq!(e0_value, Position { x: 0.0, y: 0.0 });
    }

    #[test]
    fn remove_first_entity_preserves_all_remaining_entities_values() {
        let mut world = World::new();
        let entities: Vec<EntityId> = (0..5).map(|_| world.create_entity()).collect();

        for (i, &e) in entities.iter().enumerate() {
            world
                .add_component(
                    e,
                    Position {
                        x: i as f32,
                        y: i as f32,
                    },
                )
                .unwrap();
        }

        // Remove the first-inserted entity -- forces a swap from the back
        // in a typical swap_remove(0) implementation.
        world.remove_component::<Position>(entities[0]).unwrap();

        // Every remaining entity must still map to ITS OWN original value.
        for (i, &e) in entities.iter().enumerate().skip(1) {
            let idx = world.get_entity_component_index::<Position>(e);
            assert!(
                idx.is_ok(),
                "entity {i} lost its component index after unrelated removal"
            );

            let value = world.remove_component::<Position>(e).unwrap();
            assert_eq!(
                value,
                Position {
                    x: i as f32,
                    y: i as f32
                },
                "entity {i}'s component value was corrupted by an earlier removal \
                 elsewhere in storage (classic swap_remove index-bookkeeping bug)"
            );
        }
    }

    #[test]
    fn repeated_middle_removals_never_cross_contaminate_survivors() {
        // Build up 6 entities, then remove from the middle repeatedly,
        // checking after every single removal that ALL survivors still
        // report their own correct, untouched value. This is the strongest
        // check against swap_remove corrupting bookkeeping over multiple
        // operations (as opposed to just once).
        let mut world = World::new();
        let entities: Vec<EntityId> = (0..6).map(|_| world.create_entity()).collect();
        let mut expected: HashMap<EntityId, Position> = HashMap::new();

        for (i, &e) in entities.iter().enumerate() {
            let pos = Position {
                x: i as f32 * 10.0,
                y: i as f32 * 10.0,
            };
            world.add_component(e, pos.clone()).unwrap();
            expected.insert(e, pos);
        }

        // Remove entities from the middle of the remaining set, one at a
        // time, verifying survivors after each removal.
        let removal_order = [2usize, 0, 3, 1]; // indices into `entities`, chosen up front
        for &idx in &removal_order {
            let victim = entities[idx];
            if let Some(expected_val) = expected.remove(&victim) {
                let got = world.remove_component::<Position>(victim).unwrap();
                assert_eq!(got, expected_val);
            }

            // Every remaining entity must still resolve to its own value.
            for (&survivor, survivor_expected) in expected.iter() {
                let index_result = world.get_entity_component_index::<Position>(survivor);
                assert!(
                    index_result.is_ok(),
                    "survivor lost its index after removing an unrelated entity"
                );

                // Peek the value non-destructively by temporarily removing
                // and re-adding, since there's no direct getter exposed.
                let val = world.remove_component::<Position>(survivor).unwrap();
                assert_eq!(
                    &val, survivor_expected,
                    "survivor's component was corrupted by a prior removal elsewhere \
                     (swap_remove bookkeeping bug)"
                );
                world.add_component(survivor, val).unwrap();
            }
        }
    }

    // ---- get_entity_component_index (pub(super), reachable via super::*) --

    #[test]
    fn get_entity_component_index_fails_for_invalid_entity() {
        let world = World::new();
        let bogus = EntityId(7);

        let result = world.get_entity_component_index::<Position>(bogus);
        assert!(matches!(result, Err(Error::InvalidEntityId(id)) if id == bogus));
    }

    #[test]
    fn get_entity_component_index_fails_for_unregistered_storage() {
        let mut world = World::new();
        let entity = world.create_entity();

        let result = world.get_entity_component_index::<Position>(entity);
        assert!(matches!(result, Err(Error::InvalidWorldComponent(_))));
    }

    #[test]
    fn get_entity_component_index_succeeds_after_add() {
        let mut world = World::new();
        let entity = world.create_entity();
        world
            .add_component(entity, Position { x: 5.0, y: 5.0 })
            .unwrap();

        let result = world.get_entity_component_index::<Position>(entity);
        assert!(result.is_ok());
    }

    // ---- larger-scale sanity check --------------------------------------

    #[test]
    fn many_entities_all_remain_distinct_and_valid() {
        let mut world = World::new();
        let mut ids = Vec::new();

        for _ in 0..1000 {
            ids.push(world.create_entity());
        }

        // All unique.
        let unique: HashSet<_> = ids.iter().copied().collect();
        assert_eq!(unique.len(), ids.len());

        // All valid.
        for id in ids {
            assert!(world.is_entity_valid(id));
        }
    }
}
