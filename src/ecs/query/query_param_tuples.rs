use crate::ecs::{EntityId, QueryParam, World};

impl<'w, A, B> QueryParam<'w> for (A, B) where A: QueryParam<'w>, B: QueryParam<'w> {
    type Output = (A::Output, B::Output);

    fn matches(world: &'w World, e: EntityId) -> bool {
        A::matches(world, e) && B::matches(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e))
    }
}

impl<'w, A, B, C> QueryParam<'w> for (A, B, C) where A: QueryParam<'w>, B: QueryParam<'w>, C: QueryParam<'w> {
    type Output = (A::Output, B::Output, C::Output);

    fn matches(world: &'w World, e: EntityId) -> bool {
        A::matches(world, e) && B::matches(world, e) && C::matches(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e), C::fetch(world, e))
    }
}
