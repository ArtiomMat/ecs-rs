use std::{collections::hash_map::Keys, marker::PhantomData};

use crate::ecs::{Query, entity::Entity, system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr}};


pub struct QueryIter<'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    _phantom_front_filter: PhantomData<FrontFilter>,
    _phantom_back_filter: PhantomData<BackFilter>,
    world: WorldPtr<'w>,
    inner: Option<Keys<'w, Entity, usize>>,
}

impl<'w, FrontFilter, BackFilter> QueryIter<'w, FrontFilter, BackFilter>where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter, {
    pub fn new(world: WorldPtr<'w>) -> Self {
        // SAFETY: No aliasing
        let inner = unsafe { FrontFilter::driver(world) };
        Self {
            _phantom_front_filter: PhantomData,
            _phantom_back_filter: PhantomData,
            world: world,
            inner,
        }
    }
}

impl<'w, FrontFilter, BackFilter> Iterator for QueryIter<'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    type Item = FrontFilter::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        // SAFETY: We trust that the system runner validated access to the world.
        unsafe {
            let &entity = inner.next()?;

            _ = {
                let mut state = BackFilter::create_state(self.world);
                BackFilter::filter_entity(&mut state, self.world, entity);
            };

            {
                let mut state = FrontFilter::create_state(self.world);
                FrontFilter::filter_entity(&mut state, self.world, entity)
            }
        }
    }
}
