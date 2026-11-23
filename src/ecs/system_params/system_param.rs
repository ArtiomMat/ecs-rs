use crate::ecs::system_params::world_ptr::WorldPtr;

/// A trait that any type that is a system parameter(literally placed in the function itself) must implement.
///
/// E.g. `Query`, `Global`, `Local`, etc.
pub trait SystemParam {
    type Item<'w>;

    fn get<'w>(world: WorldPtr<'w>) -> Self::Item<'w>;
}
