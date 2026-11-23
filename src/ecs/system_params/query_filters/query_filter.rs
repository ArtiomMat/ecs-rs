use std::collections::HashMap;

use crate::ecs::{entity::Entity, system_params::world_ptr::UnsafeWorldCell, world::World};

/// A trait that anything that can be a `Filter` in `Query<Filter>` must implement.
///
/// E.g. `(A: QueryFilter, B: QueryFilter)`, `&T`, `&mut T`, `Option<T: QueryFilter>`, etc.
pub trait QueryFilter {
    type State<'w>;
    type Item<'w>;

    fn create_state<'w>(world: UnsafeWorldCell<'w>) -> Self::State<'w>;
    fn driver_len(world: UnsafeWorldCell<'_>) -> usize;
    fn driver<'w> (world: UnsafeWorldCell<'w>) -> &'w HashMap<Entity, usize>;
    fn get_for_entity<'w>(state: &mut Self::State<'w>, world: UnsafeWorldCell<'w>, entity: Entity) -> Self::Item<'w>;
}
