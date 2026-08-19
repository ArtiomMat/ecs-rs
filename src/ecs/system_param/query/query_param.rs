use crate::ecs::{EntityId, World};

pub trait QueryParam<'w> {
    /// This is the more abstract part.
    ///
    /// A query parameter can fetch something, that something can be
    /// a mutable reference to a component of an entity, it can be
    /// a resource, and others, even combinations via tuples.
    ///
    /// This is essentially the heart of `QueryParam`, to fetch
    /// something when given a [`World`] reference.
    type Output;

    /// Stage I of optimization, compared against other `optimized_len()`s within a single query.
    ///
    /// Returns the smallest possible count of entities to iterate over given
    /// the information this query parameter is exposed to.
    fn optimized_len(world: &'w World) -> usize;

    /// Stage II of optimization, after findining minimal `optimized_len()` we call this on its
    /// repsective query parameter.
    ///
    /// Returns the the smallest possible iterator of entity IDs to iterate over given
    /// the information this query parameter is exposed to.
    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w>;

    /// Returns if we can fetch whatever the `QueryParam` represents
    /// given the entity ID.
    fn can_fetch(world: &'w World, entity_id: EntityId) -> bool;

    /// Returns the fetched result assuming we can fetch it.
    ///
    /// Panics if the assumption that we can fetch is broken.
    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output;
}
