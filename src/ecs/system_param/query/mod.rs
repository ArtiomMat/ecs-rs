use std::marker::PhantomData;
use crate::ecs::World;

pub use query_param::QueryParam;
pub use query_param_component::{Read, Write, Without, With};

pub mod query_param;
pub mod query_param_component;
pub mod query_param_tuples;

pub struct Query<'w, Driver: QueryParam<'w>, Filter: QueryParam<'w>> {
    world: &'w World,
    _phantom_driver: PhantomData<Driver>,
    _phantom_filter: PhantomData<Filter>,
}

impl<'w, Driver: QueryParam<'w>, Filter: 'static + QueryParam<'w>> Query<'w, Driver, Filter> {
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _phantom_driver: Default::default(),
            _phantom_filter: Default::default()
        }
    }

    pub fn query(&self) -> impl Iterator<Item = Driver::Output> {
        <(Driver, Filter)>::optimized_iter(self.world).filter(|&&entity_id| {
            <(Driver, Filter)>::can_fetch(self.world, entity_id)
        }).map(|&entity_id| {
            <Driver>::fetch(self.world, entity_id)
        })
    }
}
