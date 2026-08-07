use std::cell::{BorrowError, BorrowMutError, Ref, RefMut};

use crate::type_map::TypeMap;

#[derive(Debug)]
pub enum Error {
    Get,
    BorrowError,
}

impl From<BorrowError> for Error {
    fn from(_: BorrowError) -> Self {
        Self::BorrowError
    }
}

impl From<BorrowMutError> for Error {
    fn from(_: BorrowMutError) -> Self {
        Self::BorrowError
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Get => write!(f, "Getting"),
            Error::BorrowError => todo!(),
        }
    }
}

impl std::error::Error for Error {}

trait TypeList<'a>: Sized {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error>;
}

impl<'a, T: 'static> TypeList<'a> for Option<Ref<'a, T>> {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        match map.get::<T>()? {
            Some(storage) => Ok(Some(Ref::map(storage, |x| &x.data))),
            None => Ok(None),
        }
    }
}

impl<'a, T: 'static> TypeList<'a> for Option<RefMut<'a, T>> {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        match map.get_mut::<T>()? {
            Some(storage) => Ok(Some(RefMut::map(storage, |x| &mut x.data))),
            None => Ok(None),
        }
    }
}

impl<'a, T: 'static> TypeList<'a> for Ref<'a, T> {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        match map.get::<T>()? {
            Some(storage) => Ok(Ref::map(storage, |x| &x.data)),
            None => Err(Error::Get),
        }
    }
}

impl<'a, T: 'static> TypeList<'a> for RefMut<'a, T> {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        match map.get_mut::<T>()? {
            Some(storage) => Ok(RefMut::map(storage, |x| &mut x.data)),
            None => Err(Error::Get),
        }
    }
}

impl<'a, T: TypeList<'a>, U: TypeList<'a>> TypeList<'a> for (T, U) {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        Ok((T::get_from_map(map)?, U::get_from_map(map)?))
    }
}

impl<'a, T: TypeList<'a>, U: TypeList<'a>, V: TypeList<'a>> TypeList<'a> for (T, U, V) {
    fn get_from_map(map: &'a TypeMap) -> Result<Self, Error> {
        Ok((
            T::get_from_map(map)?,
            U::get_from_map(map)?,
            V::get_from_map(map)?,
        ))
    }
}

pub fn run_system<'a, T>(map: &'a TypeMap, system: impl Fn(T)) -> Result<(), Error>
where
    T: TypeList<'a>,
{
    T::get_from_map(map).and_then(|list| {
        system(list);
        Ok(())
    })
}
