//! Implements usage of tuples during querying for complex multi-item queries.
//!
//! How they work:
//! - They match by just &&

use crate::ecs::{EntityId, system_param::query::{QueryParam, World}};

impl<'w, A, B> QueryParam<'w> for (A, B)
where
    A: QueryParam<'w>,
    B: QueryParam<'w> + 'static,
{
    type Output = (A::Output, B::Output);

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        A::can_fetch(world, e) && B::can_fetch(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e))
    }

    fn optimized_len(world: &'w World) -> usize {
        A::optimized_len(world).min(B::optimized_len(world))
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let lens = [
            A::optimized_len(world),
            B::optimized_len(world),
        ];
        match lens.iter().enumerate().min().unwrap().0 {
            0 => A::optimized_iter(world),
            1 => B::optimized_iter(world),
            _ => unreachable!(),
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

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        A::can_fetch(world, e) && B::can_fetch(world, e) && C::can_fetch(world, e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e), C::fetch(world, e))
    }

    fn optimized_len(world: &'w World) -> usize {
        A::optimized_len(world)
            .min(B::optimized_len(world))
            .min(C::optimized_len(world))
    }


    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let lens = [
            A::optimized_len(world),
            B::optimized_len(world),
            C::optimized_len(world),
        ];
        match lens.iter().enumerate().min().unwrap().0 {
            0 => A::optimized_iter(world),
            1 => B::optimized_iter(world),
            2 => C::optimized_iter(world),
            _ => unreachable!(),
        }
    }
}
