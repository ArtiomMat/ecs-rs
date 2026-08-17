use std::marker::PhantomData;

use crate::ecs::Error;

use super::{World, ComponentsStorage};

struct Query<'a, T> {
    world: &'a World,
    phantom: PhantomData<T>,
}

impl<'a, A: 'static, B: 'static, C: 'static> Query<'a, (&A, &B, &C)> {
    pub fn new(world: &'a World) -> Result<Self, Error> {
        let component_storage_a = world.get_component_storage::<A>()?;
        let component_storage_b = world.get_component_storage::<B>()?;
        let component_storage_c = world.get_component_storage::<C>()?;

        

        Ok(Self {
            world,
            phantom: PhantomData
        })
    }
}
