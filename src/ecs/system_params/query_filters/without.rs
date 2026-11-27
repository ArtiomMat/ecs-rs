use std::{any::TypeId, collections::hash_map::Keys, marker::PhantomData};

use crate::{ecs::{
    entity::Entity, system_params::{query_filters::query_filter::QueryFilter, world_ptr::WorldPtr}, world::ValidEntityComponent,
}, mem::bit_set::{MultiBits, Op}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Without<T>(PhantomData<T>);

impl<T> QueryFilter for Without<T>
where
    T: 'static,
{
    type State<'w> = ();

    type Item<'w> = Without<T>;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w> {
        ()
    }

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>> {
        if unsafe { world.get().get_component::<T>(entity).is_err() } {
            Some(Without(PhantomData))
        } else {
            None
        }
    }

    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>> {
        // We return the set of all entities because we have no way to obtain
        // an iterator over those without T as of now.

        // SAFETY: No aliasing
        unsafe {
            world
                .get()
                .get_any_component_storage(TypeId::of::<ValidEntityComponent>())
                .map(|s| s.entities())
        }
    }

    unsafe fn push_bitset<'w>(world: WorldPtr<'w>, bits: &mut MultiBits<'w>) {
        // SAFETY: No aliasing
        if let Some(storage) = unsafe { world.get().get_any_component_storage(TypeId::of::<T>()) } {
            bits.push(storage.bitset(), Op::Difference);
        }
    }
}
