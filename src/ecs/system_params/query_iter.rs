use std::{collections::hash_map::Keys, marker::PhantomData};

use crate::ecs::{Query, entity::Entity, system_params::query_filters::query_filter::QueryFilter};

pub struct QueryIter<'a, 'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    _phantom_front_filter: PhantomData<FrontFilter>,
    _phantom_back_filter: PhantomData<BackFilter>,
    query: &'a Query<'w, FrontFilter, BackFilter>,
    driver: Option<Keys<'w, Entity, usize>>,
}

impl<'a, 'w, FrontFilter, BackFilter> QueryIter<'a, 'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    pub fn new(query: &'a Query<'w, FrontFilter, BackFilter>) -> Self {
        // SAFETY: No aliasing
        let driver = unsafe { FrontFilter::driver(query.world) };
        Self {
            _phantom_front_filter: PhantomData,
            _phantom_back_filter: PhantomData,
            query,
            driver,
        }
    }
}

impl<'w, FrontFilter, BackFilter> Iterator for QueryIter<'_, 'w, FrontFilter, BackFilter>
where
    FrontFilter: QueryFilter,
    BackFilter: QueryFilter,
{
    type Item = FrontFilter::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Check if it's actually sound, it doesn't feel right that none of this is unsafe.
        let inner = self.driver.as_mut()?;
        let &entity = inner.next()?;
        self.query.get(entity)
    }
}
