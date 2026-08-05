use std::cell::{Ref, RefMut};

use crate::type_map::TypeMap;

pub trait TypeList<'a>: Sized {
    fn get_from_map(map: &'a TypeMap) -> Option<Self>;
}

impl<'a, T: 'static> TypeList<'a> for Ref<'a, T> {
    fn get_from_map(map: &'a TypeMap) -> Option<Self> {
        map.get::<T>()
    }
}

impl<'a, T: 'static> TypeList<'a> for RefMut<'a, T> {
    fn get_from_map(map: &'a TypeMap) -> Option<Self> {
        map.get_mut::<T>()
    }
}

impl<'a, T: TypeList<'a>, U: TypeList<'a>> TypeList<'a> for (T, U) {
    fn get_from_map(map: &'a TypeMap) -> Option<Self> {
        Some((T::get_from_map(map)?, U::get_from_map(map)?))
    }
}

impl<'a, T: TypeList<'a>, U: TypeList<'a>, V: TypeList<'a>> TypeList<'a> for (T, U, V) {
    fn get_from_map(map: &'a TypeMap) -> Option<Self> {
        Some((T::get_from_map(map)?, U::get_from_map(map)?, V::get_from_map(map)?))
    }
}

pub fn run_system<'a, T>(map: &'a TypeMap, system: impl Fn(T))
where
    T: TypeList<'a>,
{
    if let Some(data) = T::get_from_map(map) {
        system(data);
    }
}
