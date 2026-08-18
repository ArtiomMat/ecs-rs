use std::{cell::{Ref, RefMut}, marker::PhantomData};

use crate::ecs::{EntityId, Error};

use super::{World, ComponentsStorage};

pub trait QueryParam<'w> {
    type Output;

    fn index(world: &'w World, entity_id: EntityId) -> Option<usize>;
    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output;
}

impl<'w, A: 'static> QueryParam<'w> for &A {
    type Output = Ref<'w, A>;

    fn index(world: &'w World, e: EntityId) -> Option<usize> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.get(&e).map(|x| *x)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.components[component_index].1.borrow()
    }
}

impl<'w, A: 'static> QueryParam<'w> for &mut A {
    type Output = RefMut<'w, A>;

    fn index(world: &'w World, e: EntityId) -> Option<usize> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.get(&e).map(|x| *x)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.components[component_index].1.borrow_mut()
    }
}

impl<'w, A, B> QueryParam<'w> for (A, B) where A: QueryParam<'w>, B: QueryParam<'w> {
    type Output = (A::Output, B::Output);

    fn index(world: &'w World, e: EntityId) -> Option<usize> {
        A::index(world, e).and(B::index(world, e))
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e))
    }
}

impl<'w, A, B, C> QueryParam<'w> for (A, B, C) where A: QueryParam<'w>, B: QueryParam<'w>, C: QueryParam<'w> {
    type Output = (A::Output, B::Output, C::Output);

    fn index(world: &'w World, e: EntityId) -> Option<usize> {
        A::index(world, e).and(B::index(world, e)).and(C::index(world, e))
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        (A::fetch(world, e), B::fetch(world, e), C::fetch(world, e))
    }
}
