use std::marker::PhantomData;

use crate::ecs::{World, system_param::system_param::SystemParam};

impl<'w, A> SystemParam<'w> for (A,) where A: SystemParam<'w> {
    fn fetch(world: &'w World) -> Self {
        Self::fetch(world)
    }
}

trait SystemParamFunction<Marker>: 'static {
    type In: for<'w> SystemParam<'w>;
    type Out;

    fn run<'w>(&mut self, param: Self::In);
}
