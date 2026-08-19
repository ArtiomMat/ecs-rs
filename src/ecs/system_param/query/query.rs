use std::marker::PhantomData;
use crate::ecs::{World, query::QueryParam};

pub struct Query<'w, Driver: QueryParam<'w>, Filter: QueryParam<'w>> {
    world: &'w World,
    _phantom_driver: PhantomData<Driver>,
    _phantom_filter: PhantomData<Filter>,
}

impl<'w, Driver: QueryParam<'w>, Filter: 'static + QueryParam<'w>> Query<'w, Driver, Filter> {
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            _phantom_driver: Default::default(),
            _phantom_filter: Default::default()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Driver::Output> {
        <(Driver, Filter)>::optimized_iter(self.world).filter(|&&entity_id| {
            <(Driver, Filter)>::can_fetch(self.world, entity_id)
        }).map(|&entity_id| {
            <Driver>::fetch(self.world, entity_id)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{With, Without, World};

    #[derive(Debug, Clone, PartialEq)]
    struct Health(i32);

    #[derive(Debug, Clone, PartialEq)]
    struct Glyph(char);

    #[derive(Debug, Clone, PartialEq)]
    struct Transform {
        x: i32,
        y: i32,
    }

    #[test]
    fn single_component_ref_query_returns_matching_entities() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(10)).unwrap();

        let e2 = world.create_entity();
        world.add_component(e2, Health(20)).unwrap();

        // Entity with no Health component at all; storage for Health still exists.
        let _e3 = world.create_entity();

        let query = Query::<&Health, ()>::new(&world);
        let mut healths: Vec<i32> = query.iter().map(|h| h.0).collect();
        healths.sort();

        assert_eq!(healths, vec![10, 20]);
    }

    #[test]
    fn single_component_mut_query_allows_mutation() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(10)).unwrap();

        let e2 = world.create_entity();
        world.add_component(e2, Health(20)).unwrap();

        {
            let query = Query::<&mut Health, ()>::new(&world);
            for mut health in query.iter() {
                health.0 += 5;
            }
        }

        let query = Query::<&Health, ()>::new(&world);
        let mut healths: Vec<i32> = query.iter().map(|h| h.0).collect();
        healths.sort();

        assert_eq!(healths, vec![15, 25]);
    }

    #[test]
    fn option_query_param_returns_none_when_missing() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(10)).unwrap();
        world.add_component(e1, Glyph('@')).unwrap();

        let e2 = world.create_entity();
        world.add_component(e2, Health(20)).unwrap();
        // e2 has no Glyph.

        // Make sure the Glyph component storage is registered even though
        // not every entity has one, otherwise `get_component_storage` would
        // error out for entities that never got a Glyph added anywhere.
        let query = Query::<(&Health, Option<&Glyph>), ()>::new(&world);
        let mut results: Vec<(i32, Option<char>)> = query
            .iter()
            .map(|(health, glyph)| (health.0, glyph.map(|g| g.0)))
            .collect();
        results.sort();

        assert_eq!(results, vec![(10, Some('@')), (20, None)]);
    }

    #[test]
    fn with_filter_restricts_to_entities_with_component() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(10)).unwrap();
        world.add_component(e1, Transform { x: 0, y: 0 }).unwrap();

        let e2 = world.create_entity();
        world.add_component(e2, Health(20)).unwrap();
        // e2 has no Transform.

        let query = Query::<&Health, With<Transform>>::new(&world);
        let healths: Vec<i32> = query.iter().map(|h| h.0).collect();

        assert_eq!(healths, vec![10]);
    }

    #[test]
    fn without_filter_excludes_entities_with_component() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(10)).unwrap();
        world.add_component(e1, Transform { x: 0, y: 0 }).unwrap();

        let e2 = world.create_entity();
        world.add_component(e2, Health(20)).unwrap();
        // e2 has no Transform.

        let query = Query::<&Health, Without<Transform>>::new(&world);
        let healths: Vec<i32> = query.iter().map(|h| h.0).collect();

        assert_eq!(healths, vec![20]);
    }

    #[test]
    fn combined_driver_and_filter_tuple_query() {
        let mut world = World::new();

        // e1: Health + Glyph + Transform -> should match.
        let e1 = world.create_entity();
        world.add_component(e1, Health(1)).unwrap();
        world.add_component(e1, Glyph('a')).unwrap();
        world.add_component(e1, Transform { x: 1, y: 1 }).unwrap();

        // e2: Health + Transform, no Glyph -> should match (Glyph is optional in filter).
        let e2 = world.create_entity();
        world.add_component(e2, Health(2)).unwrap();
        world.add_component(e2, Transform { x: 2, y: 2 }).unwrap();

        // e3: Health + Glyph, no Transform -> should NOT match (fails With<Transform>).
        let e3 = world.create_entity();
        world.add_component(e3, Health(3)).unwrap();
        world.add_component(e3, Glyph('c')).unwrap();

        // Matches the style shown in the example usage:
        // Query<(&Health, Option<&Glyph>), (Option<&Glyph>, With<Transform>)>
        let query = Query::<(&Health, Option<&Glyph>), (Option<&Glyph>, With<Transform>)>::new(&world);

        let mut results: Vec<(i32, Option<char>)> = query
            .iter()
            .map(|(health, glyph)| (health.0, glyph.map(|g| g.0)))
            .collect();
        results.sort();

        assert_eq!(results, vec![(1, Some('a')), (2, None)]);
    }

    #[test]
    fn three_tuple_query_param_intersects_all_three() {
        let mut world = World::new();

        let e1 = world.create_entity();
        world.add_component(e1, Health(1)).unwrap();
        world.add_component(e1, Glyph('a')).unwrap();
        world.add_component(e1, Transform { x: 0, y: 0 }).unwrap();

        // Missing Transform -> excluded.
        let e2 = world.create_entity();
        world.add_component(e2, Health(2)).unwrap();
        world.add_component(e2, Glyph('b')).unwrap();

        let query = Query::<(&Health, &Glyph, &Transform), ()>::new(&world);
        let results: Vec<(i32, char)> = query
            .iter()
            .map(|(health, glyph, _transform)| (health.0, glyph.0))
            .collect();

        assert_eq!(results, vec![(1, 'a')]);
    }

    #[test]
    fn query_over_empty_world_yields_nothing() {
        let world = World::new();

        // No entities and no component storages registered at all. Driver has
        // no storage registered, so `optimized_iter`/`can_fetch` for `&Health`
        // would fail via `get_component_storage`'s `.unwrap()` if entities
        // existed; with zero entities calling create_entity is unnecessary,
        // but we still need the storage registered for `&Health` to be safe.
        // Register storage without adding any entities/components.
        let mut world = world;
        world.add_component_storage::<Health>();

        let query = Query::<&Health, ()>::new(&world);
        let results: Vec<i32> = query.iter().map(|h| h.0).collect();

        assert!(results.is_empty());
    }
}
