use crate::ecs::{
    entity::Entity,
    storages::ComponentStorage,
    system_params::{query_filters::query_filter::QueryFilter, world_ptr::UnsafeWorldCell},
};

pub struct State<'w> {
    world: UnsafeWorldCell<'w>,
}

impl<T: 'static> QueryFilter for &T {
    type State<'w> = State<'w>;

    type Item<'w> = &'w T;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State<'w> {
        State {
            world: world,
        }
    }

    fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        unsafe { state.world.borrow().get_component::<T>(entity).unwrap() }
    }

    fn driver_len(world: UnsafeWorldCell<'_>) -> usize {
        0
    }

    fn driver<'w> (world: UnsafeWorldCell<'w>) -> &'w std::collections::HashMap<Entity, usize> {
        unsafe { &world.borrow().get_component_storage::<T>().expect("REMOVE").entity_to_slot }
    }
}

impl<T: 'static> QueryFilter for &mut T {
    type State<'w> = State<'w>;

    type Item<'w> = &'w mut T;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State<'w> {
        State {
            world: world,
        }
    }

    fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        unsafe { state.world.borrow_mut().get_component_mut::<T>(entity).unwrap() }
    }

    fn driver_len(world: UnsafeWorldCell<'_>) -> usize {
        todo!()
    }

    fn driver<'w> (world: UnsafeWorldCell<'w>) -> &'w std::collections::HashMap<Entity, usize> {
        todo!()
    }
}
