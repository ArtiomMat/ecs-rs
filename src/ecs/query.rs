use std::marker::PhantomData;

use super::{World, ComponentsStorage};

trait QueryParam<T, U> {
    fn fetch(storage: &ComponentsStorage<U>) -> T;
}

struct Query<'a, T> {
    world: &'a World,
    phantom: PhantomData<T>,
}

impl<'a, A> Query<'a, (&A,)> {
    
}
