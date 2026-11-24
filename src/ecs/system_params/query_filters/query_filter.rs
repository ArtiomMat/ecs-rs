use std::{any::TypeId, collections::{HashMap, hash_map::Keys}};

use crate::ecs::{entity::Entity, system_params::world_ptr::WorldPtr};

/// A trait that anything that can be a `Filter` in `Query<Filter>` must implement.
///
/// E.g. `(A: QueryFilter, B: QueryFilter)`, `&T`, `&mut T`, `Option<T: QueryFilter>`, etc.
///
/// Unsafe because it accesses `WorldPtr`.
pub trait QueryFilter {
    type State<'w>;
    type Item<'w>;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w>;

    /// Returns an exact size iterator that can be compared to others
    /// in size, and later actually iterated, for the smallest subset
    /// of entities that the query should check.
    ///
    /// # Safety
    /// The returned value immutably references `world`, you must
    /// not mutably reference `world` again until the returned
    /// value is unused.
    unsafe fn driver<'w>(world: WorldPtr<'w>) -> Option<Keys<'w, Entity, usize>>;

    unsafe fn filter_entity<'w>(
        state: &mut Self::State<'w>,
        world: WorldPtr<'w>,
        entity: Entity,
    ) -> Option<Self::Item<'w>>;
}
