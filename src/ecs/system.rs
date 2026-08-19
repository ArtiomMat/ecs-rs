use std::marker::PhantomData;

use crate::ecs::{World, system_param::system_param::SystemParam};

pub trait System {
    fn call(&mut self, world: &World);
}

// Necessary because if raw FnMut is used then we have unconstrained errors.
//
// This is yet another hack to make variadics work in Rust.
pub struct SystemFn<F, Params> {
    function: F,
    _marker: PhantomData<Params>,
}

pub trait IntoSystemFn<Params> {
    type Output;

    fn into_system(self) -> Self::Output;
}

impl<F, A> IntoSystemFn<A> for F
where
    F: FnMut(A),
    for<'w> A: SystemParam<'w>,
{
    type Output = SystemFn<F, A>;

    fn into_system(self) -> Self::Output {
        Self::Output {
            function: self,
            _marker: PhantomData,
        }
    }
}

impl<F, A> System for SystemFn<F, A>
where
    F: FnMut(A),
    for<'w> A: SystemParam<'w>,
{
    fn call(&mut self, world: &World) {
        (self.function)(A::fetch(world))
    }
}


impl<F, A, B> IntoSystemFn<(A, B)> for F
where
    F: FnMut(A, B),
    for<'w> A: SystemParam<'w>,
    for<'w> B: SystemParam<'w>,
{
    type Output = SystemFn<F, (A, B)>;

    fn into_system(self) -> Self::Output {
        Self::Output {
            function: self,
            _marker: PhantomData,
        }
    }
}

impl<F, A, B> System for SystemFn<F, (A, B)>
where
    F: FnMut(A, B),
    for<'w> A: SystemParam<'w>,
    for<'w> B: SystemParam<'w>,
{
    fn call(&mut self, world: &World) {
        (self.function)(A::fetch(world), B::fetch(world))
    }
}

impl<F, A, B, C> From<F> for SystemFn<F, (A, B, C)>
where
    F: FnMut(A, B, C),
    for<'w> A: SystemParam<'w>,
    for<'w> B: SystemParam<'w>,
    for<'w> C: SystemParam<'w>,
{
    fn from(value: F) -> Self {
        Self {
            function: value,
            _marker: PhantomData,
        }
    }
}

impl<F, A, B, C> System for SystemFn<F, (A, B, C)>
where
    F: FnMut(A, B, C),
    for<'w> A: SystemParam<'w>,
    for<'w> B: SystemParam<'w>,
    for<'w> C: SystemParam<'w>,
{
    fn call(&mut self, world: &World) {
        (self.function)(A::fetch(world), B::fetch(world), C::fetch(world))
    }
}
