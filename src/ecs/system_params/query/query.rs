use std::marker::PhantomData;

use crate::ecs::{
    entity::Entity, system_params::{query::query_param::QueryParam, system_param::SystemParam, world_ptr::UnsafeWorldCell}, world::World,
};

pub struct Query<Filter> {
    _phantom_filter: PhantomData<Filter>,
}

impl<A: QueryParam, B: QueryParam> SystemParam for Query<(A, B)> {
    type Item<'w> = (A::Item<'w>, B::Item<'w>);

    fn get<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w> {
        let a = {
            let mut state = A::create_state(world);
            A::get_for_entity(&mut state, world, entity)
        };
        let b = {
            let mut state = B::create_state(world);
            B::get_for_entity(&mut state, world, entity)
        };
        (a, b)
    }
}
