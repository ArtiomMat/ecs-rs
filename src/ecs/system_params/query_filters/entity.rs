use std::{any::TypeId, collections::hash_map::Keys};

use crate::ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
    world::ValidEntityComponent,
};

impl QueryFilter for Entity
{
    type State<'w> = ();

    type Item<'w> = Entity;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        ()
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
        Some(entity)
    }
}
