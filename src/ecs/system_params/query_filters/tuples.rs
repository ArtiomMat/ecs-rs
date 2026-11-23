use std::collections::hash_map::Keys;

use crate::ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
};

pub struct State {}

impl<A, B> QueryFilter for (A, B)
where
    A: QueryFilter,
    B: QueryFilter,
{
    type State<'w> = State;

    type Item<'w> = (A::Item<'w>, B::Item<'w>);

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        State {}
    }

    unsafe fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        unsafe {
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

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Keys<'w, Entity, usize> {
        // SAFETY: We create the slice one by one it is safe to call `driver()` sequentially.
        let min_i = unsafe { [A::driver(world).len(), B::driver(world).len()] }
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .expect("The slice is not empty");

        // SAFETY: We call `driver()` once per pattern.
        unsafe {
            match min_i {
                0 => A::driver(world),
                1 => B::driver(world),
                _ => unreachable!(),
            }
        }
    }
}
