//! Implements `Read`, `Write`, `With`, `Without`.

use std::{cell::{Ref, RefMut}, collections::HashMap};

use crate::ecs::{EntityId, QueryParam, World};


pub struct Read<T>(T);
pub struct Write<T>(T);
pub struct With<T>(T);
pub struct Without<T>(T);

impl<'w, A: 'static> QueryParam<'w> for Read<A> {
    type Output = Ref<'w, A>;

    fn matches(world: &'w World, e: EntityId) -> bool {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.contains_key(&e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.components[component_index].borrow()
    }

    fn guaranteed_matches_len(world: &'w World) -> usize {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.len()
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        Box::new(component_storage.entity_ids.iter())
    }
}

impl<'w, A: 'static> QueryParam<'w> for Write<A> {
    type Output = RefMut<'w, A>;

    fn matches(world: &'w World, e: EntityId) -> bool {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.contains_key(&e)
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.components[component_index].borrow_mut()
    }

    fn guaranteed_matches_len(world: &'w World) -> usize {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.len()
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        Box::new(component_storage.entity_ids.iter())
    }
}

impl<'w, A: 'static> QueryParam<'w> for Option<Read<A>> {
    type Output = Option<Ref<'w, A>>;

    fn matches(world: &'w World, e: EntityId) -> bool {
        world.get_component_storage::<A>().is_ok()
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        if let Ok(component_index) = component_storage.get_entity_component_index(e) {
            Some(component_storage.components[component_index].borrow())
        } else {
            None
        }
    }

    fn guaranteed_matches_len(world: &'w World) -> usize {
        world.entity_validity_set.len() // Unbounded
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter())
    }
}

impl<'w, A: 'static> QueryParam<'w> for With<A> {
    type Output = ();

    fn matches(world: &'w World, e: EntityId) -> bool {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.contains_key(&e)
    }

    fn fetch(_world: &'w World, _e: EntityId) -> Self::Output {
        ()
    }

    fn guaranteed_matches_len(world: &'w World) -> usize {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_component.len()
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        Box::new(component_storage.entity_ids.iter())
    }
}

impl<'w, A: 'static> QueryParam<'w> for Without<A> {
    type Output = ();

    fn matches(world: &'w World, e: EntityId) -> bool {
        !With::<A>::matches(world, e)
    }

    fn fetch(_world: &'w World, _e: EntityId) -> Self::Output {
        ()
    }

    fn guaranteed_matches_len(world: &'w World) -> usize {
        world.entity_validity_set.len() // Unbounded
    }

    fn guaranteed_matches_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter())
    }
}
