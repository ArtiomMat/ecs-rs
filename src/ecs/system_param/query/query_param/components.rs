//! Implements `Read`, `Write`, `With`, `Without`.

use crate::ecs::{EntityId, World, system_param::query::QueryParam};
use std::cell::{Ref, RefMut};

pub struct With<T>(T);
pub struct Without<T>(T);

impl<'w, A: 'static> QueryParam<'w> for &A {
    type Output = Ref<'w, A>;

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        match world.get_component_storage::<A>() {
            Some(component_storage) => component_storage.entity_id_to_item.contains_key(&e),
            None => false,
        }
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.items[component_index].component.borrow()
    }

    fn optimized_len(world: &'w World) -> usize {
        match world.get_component_storage::<A>() {
            Some(component_storage) => component_storage.len(),
            None => 0,
        }
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        match world.get_component_storage::<A>() {
            Some(component_storage) => Box::new(component_storage.items.iter().map(|c| &c.entity_id)),
            None => Box::new([].into_iter()),
        }
    }
}

impl<'w, A: 'static> QueryParam<'w> for &mut A {
    type Output = RefMut<'w, A>;

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        match world.get_component_storage::<A>() {
            Some(component_storage) => component_storage.entity_id_to_item.contains_key(&e),
            None => false,
        }
    }

    fn fetch(world: &'w World, e: EntityId) -> Self::Output {
        let component_storage = world.get_component_storage().unwrap();
        let component_index = component_storage.get_entity_component_index(e).unwrap();
        component_storage.items[component_index]
            .component
            .borrow_mut()
    }

    fn optimized_len(world: &'w World) -> usize {
        match world.get_component_storage::<A>() {
            Some(component_storage) => component_storage.len(),
            None => 0,
        }
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        match world.get_component_storage::<A>() {
            Some(component_storage) => Box::new(component_storage.items.iter().map(|c| &c.entity_id)),
            None => Box::new([].into_iter()),
        }
    }
}

impl<'w, A: QueryParam<'w> + 'static> QueryParam<'w> for Option<A> {
    type Output = Option<A::Output>;

    fn can_fetch(_world: &'w World, _entity_id: EntityId) -> bool {
        true
    }

    fn fetch(world: &'w World, entity_id: EntityId) -> Self::Output {
        if A::can_fetch(world, entity_id) {
            Some(A::fetch(world, entity_id))
        } else {
            None
        }
    }

    fn optimized_len(world: &'w World) -> usize {
        world.entity_validity_set.len() // Unbounded
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter()) // Unbounded
    }
}

impl<'w, A: 'static> QueryParam<'w> for With<A> {
    type Output = ();

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.entity_id_to_item.contains_key(&e)
    }

    fn fetch(_world: &'w World, _e: EntityId) -> Self::Output {
        ()
    }

    fn optimized_len(world: &'w World) -> usize {
        let component_storage = world.get_component_storage::<A>().unwrap();
        component_storage.len()
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        let component_storage = world.get_component_storage::<A>().unwrap();
        Box::new(component_storage.items.iter().map(|c| &c.entity_id))
    }
}

impl<'w, A: 'static> QueryParam<'w> for Without<A> {
    type Output = ();

    fn can_fetch(world: &'w World, e: EntityId) -> bool {
        !With::<A>::can_fetch(world, e)
    }

    fn fetch(_world: &'w World, _e: EntityId) -> Self::Output {
        ()
    }

    fn optimized_len(world: &'w World) -> usize {
        world.entity_validity_set.len() // Unbounded
    }

    fn optimized_iter(world: &'w World) -> Box<dyn Iterator<Item = &'w EntityId> + 'w> {
        Box::new(world.entity_validity_set.iter()) // Unbounded
    }
}
