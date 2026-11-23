use std::{marker::PhantomData, sync::mpsc::Iter};

use crate::ecs::{
    entity::Entity,
    system_params::{
        query_filters::query_filter::QueryFilter,
        system_param::SystemParam, world_ptr::UnsafeWorldCell,
    },
    world::World,
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
    world: UnsafeWorldCell<'w>,
}

impl<FrontFilter, BackFilter> SystemParam for Query<'_, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    fn get<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
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
    pub fn query(&mut self) -> FrontFilter::Item<'w> {
        for (&entity, _) in FrontFilter::driver(self.world) {
            let a = {
                let mut state = FrontFilter::create_state(self.world);
                FrontFilter::get_for_entity(&mut state, self.world, entity)
            };
            {
                let mut state = BackFilter::create_state(self.world);
                _ = BackFilter::get_for_entity(&mut state, self.world, entity);
            };
            return a;
        }

        todo!()
    }
}
