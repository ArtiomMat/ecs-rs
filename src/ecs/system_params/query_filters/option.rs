use std::{any::TypeId, collections::hash_map::Keys};

use crate::{ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
    world::ValidEntityComponent,
}, mem::bit_set::MultiBits};

impl<T> QueryFilter for Option<T>
where
    T: QueryFilter,
{
    type State<'w> = T::State<'w>;

    type Item<'w> = Option<T::Item<'w>>;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        unsafe { T::create_state(world) }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>> {
        // Driver is the set of all entities.

        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_any_component_storage(TypeId::of::<ValidEntityComponent>())
                .map(|s| s.entities())
        }
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        // Option always succeeds, it is up to T to decide the inner option.

        Some(unsafe { T::filter_entity(state, world, entity) })
    }

    unsafe fn push_bitset<'w>(_: WorldPtr<'w>, _: &mut MultiBits<'w>) {
        // Nothing is equivalent to all intersecting all 1 set
    }
}
