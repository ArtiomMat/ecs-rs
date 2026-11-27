use std::marker::PhantomData;

use crate::ecs::{
    entity::Entity,
    system_params::{
        query_filters::query_filter::QueryFilter, query_iter::QueryIter, system_param::SystemParam,
        world_ptr::WorldPtr,
    },
};

/// A system parameter that allows:
/// - Iterating over entities that conform to a certain filter tuple.
/// - Checking if specific entities.
///
/// `FrontFilter`'s result will be the one visible to you during querying.
/// `BackFilter`'s result is used in the background, to not clutter your query result.
pub struct Query<'w, FrontFilter>
where
    FrontFilter: QueryFilter,
{
    _phantom_filter: PhantomData<FrontFilter>,
    pub(super) world: WorldPtr<'w>,
}

impl<Filter> SystemParam for Query<'_, Filter>
where
    Filter: QueryFilter,
{
    type Item<'w> = Query<'w, Filter>;

    unsafe fn fetch<'w>(world: WorldPtr<'w>) -> Self::Item<'w> {
        Query {
            _phantom_filter: Default::default(),
            world: world,
        }
    }
}

impl<'w, Filter> Query<'w, Filter>
where
    Filter: QueryFilter,
{
    pub fn get(&self, entity: Entity) -> Option<Filter::Item<'w>> {
        // SAFETY: We trust that the system runner validated access to the world.
        unsafe {
            {
                let mut state = Filter::create_state(self.world);
                Filter::filter_entity(&mut state, self.world, entity)
            }
        }
    }

    pub fn iter<'a>(&'a self) -> QueryIter<'a, 'w, Filter> {
        QueryIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    //! Tests here focus on valid inputs, ensuring they don't panic or UB(Which needs Miri but can be revealed via crashes)

    use crate::ecs::{self, World};
    use super::*;

    #[test]
    fn i8_and_i16_combinations() {
        let mut world = World::new();

        // Will work
        let e0 = world.spawn();
        world.insert_component(e0, 1i8).unwrap();
        world.insert_component(e0, 2i16).unwrap();

        // Wont work
        let e1 = world.spawn();
        world.insert_component(e1, 3i8).unwrap();

        // Will work
        let e2 = world.spawn();
        world.insert_component(e2, 4i8).unwrap();
        world.insert_component(e2, 5i16).unwrap();
        world.insert_component(e2, 6i32).unwrap();

        let world_ptr = WorldPtr::new(&mut world);

        let query: Query<(&i8, &i16)> = unsafe { Query::fetch(world_ptr) };

        assert_eq!(query.get(e0), Some((&1, &2)));
        assert_eq!(query.get(e1), None);
        assert_eq!(query.get(e2), Some((&4, &5)));
    }

    #[test]
    fn i8_without_i16_combinations() {
        let mut world = World::new();

        // Will work
        let e0 = world.spawn();
        world.insert_component(e0, 1i8).unwrap();
        world.insert_component(e0, 2i16).unwrap();

        // Wont work
        let e1 = world.spawn();
        world.insert_component(e1, 3i8).unwrap();

        // Will work
        let e2 = world.spawn();
        world.insert_component(e2, 4i8).unwrap();
        world.insert_component(e2, 5i16).unwrap();
        world.insert_component(e2, 6i32).unwrap();

        // Will work
        let e3 = world.spawn();
        world.insert_component(e3, 7i8).unwrap();
        world.insert_component(e3, 8i32).unwrap();

        let world_ptr = WorldPtr::new(&mut world);

        let query: Query<(&i8, ecs::Without<i16>)> = unsafe { Query::fetch(world_ptr) };

        assert_eq!(query.get(e0), None);
        assert_eq!(query.get(e1).unwrap().0, &3i8);
        assert_eq!(query.get(e2), None);
        assert_eq!(query.get(e3).unwrap().0, &7i8);
    }
}
