//! Implements usage of tuples during querying for complex multi-item queries.

use crate::ecs::{EntityId, QueryParam, World};

impl<'w, A, B> QueryParam<'w> for (A, B)
where
    A: QueryParam<'w>,
    B: QueryParam<'w> + 'static,
{
    type Output = (A::Output, B::Output);

    fn matches(world: &'w World, e: EntityId) -> bool {
        A::matches(world, e) && B::matches(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e))
    }

    fn guaranteed_matches_len(world: &'w World) -> Option<usize> {
        A::guaranteed_matches_len(world).min(B::guaranteed_matches_len(world))
    }

    fn guaranteed_matches_iter(world: &'w World) -> Option<std::slice::Iter<'w, EntityId>> {
        let lens = [
            A::guaranteed_matches_len(world),
            B::guaranteed_matches_len(world),
        ];
        match lens.iter().enumerate().min() {
            Some((i, ..)) => match i {
                0 => A::guaranteed_matches_iter(world),
                1 => B::guaranteed_matches_iter(world),
                _ => None,
            },
            None => None,
        }
    }
}

impl<'w, A, B, C> QueryParam<'w> for (A, B, C)
where
    A: QueryParam<'w>,
    B: QueryParam<'w>,
    C: QueryParam<'w>,
{
    type Output = (A::Output, B::Output, C::Output);

    fn matches(world: &'w World, e: EntityId) -> bool {
        A::matches(world, e) && B::matches(world, e) && C::matches(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e), C::fetch(world, e))
    }

    fn guaranteed_matches_len(world: &'w World) -> Option<usize> {
        A::guaranteed_matches_len(world)
            .min(B::guaranteed_matches_len(world))
            .min(C::guaranteed_matches_len(world))
    }

    fn guaranteed_matches_iter(world: &'w World) -> Option<std::slice::Iter<'w, EntityId>> {
        let lens = [
            A::guaranteed_matches_len(world),
            B::guaranteed_matches_len(world),
            C::guaranteed_matches_len(world),
        ];
        match lens.iter().enumerate().min() {
            Some((i, ..)) => match i {
                0 => A::guaranteed_matches_iter(world),
                1 => B::guaranteed_matches_iter(world),
                2 => C::guaranteed_matches_iter(world),
                _ => None,
            },
            None => None,
        }
    }
}
