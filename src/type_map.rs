use core::any::{Any, TypeId};
use std::{
    cell::{BorrowError, Ref, RefCell, RefMut, UnsafeCell}, collections::HashMap,
};

use crate::system_runner::Error;

pub struct TypeStorage<T> {
    pub data: T,
}

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
            .insert(TypeId::of::<V>(), Box::new(RefCell::new(TypeStorage::<V>{data: v})));
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

    pub fn get<'a, V: 'static>(&'a self) -> Result<Option<Ref<'a, TypeStorage<V>>>, Error> {
        // TODO: Turn
        let g = self.inner.get(&TypeId::of::<V>()).ok_or(Error::Get)?;
        let g = g.try_borrow()?;
        Ok(Some(Ref::filter_map(g, |x| x.downcast_ref()).unwrap()))
    }

    pub fn get_mut<'a, V: 'static>(&'a self) -> Result<Option<RefMut<'a, TypeStorage<V>>>, Error> {
        let g = self.inner.get(&TypeId::of::<V>()).ok_or(Error::Get)?;
        let g = g.try_borrow_mut()?;
        Ok(Some(RefMut::filter_map(g, |x| x.downcast_mut()).unwrap()))
    }
}
