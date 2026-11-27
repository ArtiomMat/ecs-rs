use std::{collections::hash_map::Keys, marker::PhantomData};

use crate::{
    ecs::{
        Query,
        entity::Entity,
        storages::{AnyComponentStorage, ComponentStorage},
        system_params::query_filters::query_filter::QueryFilter,
        world::ValidEntityComponent,
    },
    mem::bit_set::{MultiBits, Op::Intersection},
};

enum Startegy<'w> {
    Driver(Keys<'w, Entity, usize>),
    BitSet(MultiBits<'w>),
}

pub struct QueryIter<'a, 'w, Filter>
where
    Filter: QueryFilter,
{
    _phantom_filter: PhantomData<Filter>,
    query: &'a Query<'w, Filter>,
    strategy: Startegy<'w>,
}

impl<'a, 'w, Filter> QueryIter<'a, 'w, Filter>
where
    Filter: QueryFilter,
{
    unsafe fn get_valid_entity_component_storage(
        query: &'a Query<'w, Filter>,
    ) -> &'w ComponentStorage<ValidEntityComponent> {
        unsafe {
            query
                .world
                .get()
                .get_component_storage::<ValidEntityComponent>()
                .unwrap()
        }
    }

    pub fn new(query: &'a Query<'w, Filter>) -> Self {
        // SAFETY: No aliasing
        let driver = unsafe {
            Filter::driver(query.world)
                .unwrap_or(Self::get_valid_entity_component_storage(query).entities())
        };

        let mut bits = MultiBits::new();
        // SAFETY: No aliasing
        unsafe { Filter::push_bitset(query.world, &mut bits) };
        if bits.pushed_count() == 0 {
            bits.push(
                unsafe { Self::get_valid_entity_component_storage(query).bitset() },
                Intersection,
            );
        }

        // Choose strategy, if the driver method causes less lookups prefer that.
        let strategy = if driver.len() < bits.max_capacity() {
            Startegy::Driver(driver)
        } else {
            Startegy::BitSet(bits)
        };

        Self {
            _phantom_filter: PhantomData,
            query,
            strategy,
        }
    }
}

impl<'w, Filter> Iterator for QueryIter<'_, 'w, Filter>
where
    Filter: QueryFilter,
{
    type Item = Filter::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Check if it's actually sound, it doesn't feel right that none of this is unsafe.

        let entity = match &mut self.strategy {
            Startegy::Driver(keys) => keys.next().cloned(),
            Startegy::BitSet(bits) => bits.next().and_then(|b| {
                // If the return value is None then we should have reached a stale bit resulting from a Difference or something.
                // This will just propagate, we are fine.
                unsafe { self.query.world.get().index_to_entity(b) }
            }),
        }?;

        self.query.get(entity)
    }
}

#[cfg(test)]
mod tests {
    //! The iterator is an optimization running over Query, and so it should be treated as a separate way to query
    //! entities because it may accidentally break existing query matching through its wrapping.
    //!
    //! Tests here primarily aim to ensure that:
    //! - We don't panic.
    //! - We don't cause UB on valid inputs(Which needs Miri but can be revealed via crashes).
    //! - We always return matching entities.
    //! - We never return non-matching entities.

    use crate::ecs::{World, system_params::{system_param::SystemParam, world_ptr::WorldPtr}};

    use super::*;

    #[test]
    fn empty() {
        let mut world = World::new();

        let world_ptr = WorldPtr::new(&mut world);
        let query: Query<(&i8, &i16)> = unsafe { Query::fetch(world_ptr) };
        let mut iter = QueryIter::new(&query);

        assert_eq!(iter.next(), None)
    }

    #[test]
    fn stale() {
        todo!("Add many entities then remove them and see that we don't get stale ones or panic")
    }

    #[test]
    fn random_placement() {
        todo!("Add many entities then remove them and see that we don't get stale ones or panic")
    }
}
