//! Implements getting the entity ID

use crate::ecs::{EntityId, World, query::QueryParam};

impl<'w> QueryParam<'w> for EntityId {
    type Output = EntityId;

    fn can_fetch(_world: &'w World, _entity_id: EntityId) -> bool {
        true
    }

    fn fetch(_world: &'w World, entity_id: EntityId) -> Self::Output {
        entity_id
    }

    fn optimized_len(world: &'w World) -> usize {
        world.entity_validity_set.len() // Everything
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter()) // Everything
    }
}
