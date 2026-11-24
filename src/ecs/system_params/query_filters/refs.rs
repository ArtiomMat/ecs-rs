use std::{
    any::TypeId,
    collections::hash_map::{self, Keys},
};

use crate::ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
};

// TODO: Remove world.
pub struct State<'w> {
    world: WorldPtr<'w>,
}

impl<T> QueryFilter for &T
where
    T: 'static,
{
    type State<'w> = State<'w>;

    type Item<'w> = &'w T;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        State { world: world }
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        unsafe { state.world.get().get_component::<T>(entity).ok() }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>> {
        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_any_component_storage(TypeId::of::<T>())
                .map(|x| x.entities())
        }
    }
}

impl<T> QueryFilter for &mut T
where
    T: 'static,
{
    type State<'w> = State<'w>;

    type Item<'w> = &'w mut T;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        State { world: world }
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        unsafe { state.world.get_mut().get_component_mut::<T>(entity).ok() }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>> {
        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_any_component_storage(TypeId::of::<T>())
                .map(|x| x.entities())
        }
    }
}
