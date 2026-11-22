use crate::ecs::{entity::Entity, storages::ComponentStorage, system_params::{query::query_param::QueryParam, world_ptr::UnsafeWorldCell}};

pub struct State {
}

impl<T: 'static> QueryParam for &T {
    type State = State;

    type Item<'w> = &'w T;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State {
        State{}
    }

    fn get_for_entity<'w>(state: &mut Self::State, world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w> {
        unsafe {
            world.borrow().get_component::<T>(entity).unwrap()
        }
    }
}

impl<T: 'static> QueryParam for &mut T {
    type State = State;

    type Item<'w> = &'w mut T;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State {
        State{}
    }

    fn get_for_entity<'w>(state: &mut Self::State, world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w> {
        unsafe {
            world.borrow_mut().get_component_mut::<T>(entity).unwrap()
        }
    }
}
