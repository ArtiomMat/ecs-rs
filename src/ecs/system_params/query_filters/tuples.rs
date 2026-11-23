use crate::ecs::{entity::Entity, system_params::{query_filters::query_filter::QueryFilter, world_ptr::UnsafeWorldCell}};

pub struct State{}

impl<A, B> QueryFilter for (A, B)
where
    A: QueryFilter,
    B: QueryFilter,
{
    type State<'w> = State;

    type Item<'w> = (A::Item<'w>, B::Item<'w>);

    fn create_state<'w>(
        world: UnsafeWorldCell<'w>,
    ) -> Self::State<'w> {
        State{}
    }

    fn driver_len(world: UnsafeWorldCell<'_>) -> usize {
        A::driver_len(world)
    }

    fn driver<'w>(
        world: UnsafeWorldCell<'w>,
    ) -> &'w std::collections::HashMap<Entity, usize> {
        A::driver(world)
    }

    fn get_for_entity<'w>(
        state: &mut Self::State<'w>,
        world: UnsafeWorldCell<'w>,
        entity: Entity,
    ) -> Self::Item<'w> {
        let a = {
            let mut state = A::create_state(world);
            A::get_for_entity(&mut state, world, entity)
        };
        let b = {
            let mut state = B::create_state(world);
            B::get_for_entity(&mut state, world, entity)
        };
        (a, b)
    }
}
