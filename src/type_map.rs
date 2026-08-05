use core::any::{Any, TypeId};
use std::{
    cell::{Ref, RefCell, RefMut, UnsafeCell},
    collections::HashMap,
};

/// # Example
///
/// ```
/// let mut map = TypeMap::new();
/// map.insert(42i32);
/// map.insert("hello".to_string());
///
/// assert_eq!(map.get::<i32>(), Some(&42));
/// assert_eq!(map.get::<String>(), Some(&"hello".to_string()));
/// ```
pub struct TypeMap {
    inner: HashMap<TypeId, Box<RefCell<dyn Any>>>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert<V: 'static>(&mut self, v: V) {
        self.inner
            .insert(TypeId::of::<V>(), Box::new(RefCell::new(v)));
    }

    // pub unsafe fn get<'a, V: 'static>(&'a self) -> Option<&'a V> {
    //     // TODO: Turn
    //     self.inner
    //         .get(&TypeId::of::<V>())
    //         .and_then(|x| (unsafe { &*(x.get()) }).downcast_ref())
    // }

    // pub unsafe fn get_mut<'a, V: 'static>(&'a self) -> Option<&'a mut V> {
    //     self.inner
    //         .get(&TypeId::of::<V>())
    //         .and_then(|x| (unsafe { &mut *(x.get()) }).downcast_mut())
    // }

    pub fn get<'a, V: 'static>(&'a self) -> Option<Ref<'a, V>> {
        // TODO: Turn
        self.inner
            .get(&TypeId::of::<V>())
            .and_then(|x| x.try_borrow().ok())
            .and_then(|x| Ref::filter_map(x, |x| x.downcast_ref()).ok())
    }

    pub fn get_mut<'a, V: 'static>(&'a self) -> Option<RefMut<'a, V>> {
        self.inner
            .get(&TypeId::of::<V>())
            .and_then(|x| x.try_borrow_mut().ok())
            .and_then(|x| RefMut::filter_map(x, |x| x.downcast_mut()).ok())
    }
}
