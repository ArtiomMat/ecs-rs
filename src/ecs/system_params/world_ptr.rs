use std::marker::PhantomData;

use crate::ecs::world::{World};

/// Passed to system parameters(`Query` also passes it to its filters).
/// Ideally we would prefer to pass `&mut World` but Rust fights us a lot when we try.
///
/// It works by borrowing `&mut World`, meaning that while we use it the original instance
/// of the world is considered mutably borrowed.
/// From there, as long as we don't do dumb shit, we wont violate the aliasing rules.
#[derive(Clone, Copy)]
pub struct WorldPtr<'w>(*mut World, PhantomData<&'w mut World>);

impl<'w> WorldPtr<'w> {
    pub fn new(world: &mut World) -> Self {
        Self(world as *mut World, Default::default())
    }

    /// # Safety
    /// There must be no other aliases alive at call site.
    pub unsafe fn get_mut(self) -> &'w mut World {
        // SAFETY: Caller must obey aliasing rules
        unsafe { &mut *self.0 }
    }

    /// # Safety
    /// There can be other aliases, as long as they are not mutable, at call site.
    pub unsafe fn get(self) -> &'w World {
        // SAFETY: Caller must obey aliasing rules
        unsafe { &*self.0 }
    }
}
