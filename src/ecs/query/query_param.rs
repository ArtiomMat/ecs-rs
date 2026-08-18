use std::collections::HashMap;

use crate::ecs::{EntityId, World};

pub trait QueryParam<'w> {
    type Output;

    fn guaranteed_matches_len(world: &'w World) -> usize {
        world.entity_validity_set.len()
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter())
    }

    fn matches(world: &'w World, entity_id: EntityId) -> bool;
    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output;
}
