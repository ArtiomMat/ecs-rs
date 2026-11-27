use std::collections::hash_map::Keys;

use crate::{
    ecs::{
        entity::Entity,
        system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
    },
    mem::bit_set::MultiBits,
};

impl<A, B> QueryFilter for (A, B)
where
    A: QueryFilter,
    B: QueryFilter,
{
    type State<'w> = ();

    type Item<'w> = (A::Item<'w>, B::Item<'w>);

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        ()
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        unsafe {
            let a = {
                let mut state = A::create_state(world);
                A::filter_entity(&mut state, world, entity)?
            };
            let b = {
                let mut state = B::create_state(world);
                B::filter_entity(&mut state, world, entity)?
            };
            Some((a, b))
        }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>> {
        // SAFETY: We create the slice one by one it is safe to call `driver()` sequentially.
        let min_i = unsafe { [A::driver(world), B::driver(world)] }
            .iter()
            .enumerate()
            .min_by_key(|&(_index, driver)| driver.as_ref().map(|d| d.len()))
            .map(|(index, _driver)| index)?;

        // SAFETY: We call `driver()` once per pattern.
        unsafe {
            match min_i {
                0 => A::driver(world),
                1 => B::driver(world),
                _ => unreachable!(),
            }
        }
    }

    unsafe fn push_bitset<'w>(world: WorldPtr<'w>, bits: &mut MultiBits<'w>) {
        unsafe {
            A::push_bitset(world, bits);
            B::push_bitset(world, bits);
        }
    }
}
