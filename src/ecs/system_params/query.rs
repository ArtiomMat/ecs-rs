use std::{collections::hash_map::Keys, marker::PhantomData};

use crate::ecs::{
    entity::Entity, system_params::{
        query_filters::query_filter::QueryFilter, query_iter::QueryIter, system_param::SystemParam, world_ptr::WorldPtr,
    },
};

/// A system parameter that allows:
/// - Iterating over entities that conform to a certain filter tuple.
/// - Checking if specific entities.
///
/// `FrontFilter`'s result will be the one visible to you during querying.
/// `BackFilter`'s result is used in the background, to not clutter your query result.
pub struct Query<'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    _phantom_front_filter: PhantomData<FrontFilter>,
    _phantom_back_filter: PhantomData<BackFilter>,
    world: WorldPtr<'w>,
}

impl<FrontFilter, BackFilter> SystemParam for Query<'_, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    fn fetch<'w>(world: WorldPtr<'w>) -> Self::Item<'w> {
        Query {
            _phantom_back_filter: Default::default(),
            _phantom_front_filter: Default::default(),
            world: world,
        }
    }

    type Item<'w> = Query<'w, FrontFilter, BackFilter>;
}

impl<'w, FrontFilter, BackFilter> Query<'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    pub fn get(&mut self, entity: Entity) -> Option<FrontFilter::Item<'w>> {
        // SAFETY: We trust that the system runner validated access to the world.
        unsafe {
            _ = {
                let mut state = BackFilter::create_state(self.world);
                _ = BackFilter::filter_entity(&mut state, self.world, entity);
            };
            {
                let mut state = FrontFilter::create_state(self.world);
                FrontFilter::filter_entity(&mut state, self.world, entity)
            }
        }
    }

    pub fn iter(&self) -> QueryIter<'w, FrontFilter, BackFilter> {
        QueryIter::new(self.world)
    }
}
