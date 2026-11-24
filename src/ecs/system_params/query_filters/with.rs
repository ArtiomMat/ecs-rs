use std::{any::TypeId, collections::hash_map::Keys, marker::PhantomData};

use crate::ecs::{
    entity::Entity,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr},
};

pub struct With<T>(PhantomData<T>);

impl<T> QueryFilter for With<T>
where
    T: 'static,
{
    type State<'w> = ();

    type Item<'w> = With<T>;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        ()
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        if unsafe { world.get().get_component::<T>(entity).is_ok() } {
            Some(With(PhantomData))
        } else {
            None
        }
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
