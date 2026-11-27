use std::collections::hash_map::Keys;

use crate::{
    ecs::{entity::Entity, system_params::world_ptr::WorldPtr},
    mem::bit_set::MultiBits,
};

/// A trait that anything that can be a `Filter` in `Query<Filter>` must implement.
///
/// E.g. `(A: QueryFilter, B: QueryFilter)`, `&T`, `&mut T`, `Option<T: QueryFilter>`, etc.
///
/// Unsafe because it accesses `WorldPtr`.
pub trait QueryFilter {
    type State<'w>;
    type Item<'w>;

    unsafe fn create_state<'w>(world: WorldPtr<'w>) -> Self::State<'w>;

    /// Returns a bitset reference + operation.
    /// This is used for bitset intersection based iteration over entities.
    ///
    /// May or may not be utilized depending on the query iterator's logic.
    /// Optional anyway.
    ///
    /// # Safety
    /// The returned value immutably references `world`, you must
    /// not mutably reference `world` again until the returned
    /// value is unused.
    unsafe fn push_bitset<'w>(world: WorldPtr<'w>, bits: &mut MultiBits<'w>) {}

    /// Returns an exact size iterator that can be compared to others
    /// in size, and later actually iterated, for the smallest subset
    /// of entities that the query should check.
    ///
    /// May or may not be utilized depending on the query iterator's logic.
    /// Optional anyway.
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
