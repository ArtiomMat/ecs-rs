use crate::ecs::{entity::Entity, system_params::world_ptr::UnsafeWorldCell, world::World};

/// A trait that any type that is a system parameter(literally placed in the function itself) must implement.
///
/// E.g. `Query`, `Global`, `Local`, etc.
pub trait SystemParam {
    type Item<'w>;

    fn get<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w>;
}
