use crate::ecs::{entity::Entity, system_params::world_ptr::UnsafeWorldCell, world::World};

pub trait SystemParam {
    type Item<'w>;

    fn get<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w>;
}
