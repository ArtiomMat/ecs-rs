use crate::ecs::{EntityId, World};

pub trait QueryParam<'w> {
    type Output;

    fn matches(world: &'w World, entity_id: EntityId) -> bool;
    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output;
}
