use std::marker::PhantomData;

use crate::ecs::system_params::{system_param::SystemParam, world_ptr::WorldPtr};

pub trait System<Marker> {
    type Result;

    unsafe fn run(&mut self, world: WorldPtr) -> Self::Result;
}

impl<F, A, R> System<PhantomData<fn(A) -> R>> for F
where
    F: FnMut(A) -> R,
    F: for<'a> FnMut(A::Item<'a>) -> R,
    A: SystemParam,
{
    type Result = R;

    unsafe fn run(&mut self, world: WorldPtr) -> Self::Result {
        unsafe { self(A::fetch(world)) }
    }
}

impl<F, A, B, R> System<PhantomData<fn(A, B) -> R>> for F
where
    F: FnMut(A, B) -> R,
    F: for<'a> FnMut(A::Item<'a>, B:: Item<'a>) -> R,
    A: SystemParam,
    B: SystemParam,
{
    type Result = R;

    unsafe fn run(&mut self, world: WorldPtr) -> Self::Result {
        unsafe { self(A::fetch(world), B::fetch(world)) }
    }
}
