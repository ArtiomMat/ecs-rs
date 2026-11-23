use std::marker::PhantomData;

use crate::ecs::world::{World};

/// Passed to system parameters(`Query` also passes it to its filters).
#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w>(*mut World, PhantomData<&'w World>);

impl<'w> UnsafeWorldCell<'w> {
    pub unsafe fn new(world: &mut World) -> Self {
        Self(world as *mut World, Default::default())
    }

    pub unsafe fn borrow_mut(self) -> &'w mut World {
        unsafe { &mut *self.0 }
    }

    pub unsafe fn borrow(self) -> &'w World {
        unsafe { &*self.0 }
    }
}
