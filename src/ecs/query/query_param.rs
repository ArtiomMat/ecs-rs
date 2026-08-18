use std::collections::HashMap;

use crate::ecs::{EntityId, World};

pub trait QueryParam<'w> {
    type Output;

    fn matches_len(world: &'w World) -> Option<usize> {
        None
    }

    fn matches(world: &'w World, entity_id: EntityId) -> bool;
    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output;
}
