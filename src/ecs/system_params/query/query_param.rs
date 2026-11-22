use crate::ecs::{entity::Entity, system_params::world_ptr::UnsafeWorldCell, world::World};

pub trait QueryParam {
    type State;
    type Item<'w>;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State;
    fn get_for_entity<'w>(state: &mut Self::State, world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w>;
}
