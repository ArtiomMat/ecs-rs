use std::{any::TypeId, collections::hash_map};

use crate::ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
};

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

    unsafe fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        unsafe { state.world.get().get_component::<T>(entity).unwrap() }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> hash_map::Keys<'w, Entity, usize> {
        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_untyped_component_storage(TypeId::of::<T>())
                .expect("REMOVE FOR OPTION<T>")
                .entities()
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

    unsafe fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        unsafe {
            state
                .world
                .get_mut()
                .get_component_mut::<T>(entity)
                .expect("REMOVE AND PUT OPTION<T>")
        }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> hash_map::Keys<'w, Entity, usize> {
        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_untyped_component_storage(TypeId::of::<T>())
                .expect("REMOVE FOR OPTION<T>")
                .entities()
        }
    }
}
