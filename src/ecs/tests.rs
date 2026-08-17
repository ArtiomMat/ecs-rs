// #[cfg(test)]
// mod tests {
//     use crate::ecs::{EntityId, World};

//     struct PositionComponent([i32; 3]);
//     struct HealthComponent(i32);
//     struct PlayerTag;
//     struct EnemyTag;

//     // is_entity_valid

//     #[test]
//     fn created_entity_is_valid() {
//         let mut world = World::new();
//         let id = world.create_entity();
//         assert!(world.is_entity_valid(id));
//     }

//     #[test]
//     fn fabricated_entity_id_is_invalid() {
//         let world = World::new();
//         // No entities have been created, so any EntityId is bogus.
//         let fake = EntityId(9999);
//         assert!(!world.is_entity_valid(fake));
//     }

//     #[test]
//     fn multiple_created_entities_are_all_valid() {
//         let mut world = World::new();
//         let ids: Vec<_> = (0..5).map(|_| world.create_entity()).collect();
//         for id in ids {
//             assert!(world.is_entity_valid(id));
//         }
//     }

//     #[test]
//     fn created_entities_have_distinct_ids() {
//         let mut world = World::new();
//         let a = world.create_entity();
//         let b = world.create_entity();
//         let c = world.create_entity();
//         assert_ne!(a, b);
//         assert_ne!(b, c);
//         assert_ne!(a, c);
//     }

//     // add_component_storage

//     #[test]
//     fn add_component_storage_returns_false_on_first_registration() {
//         let mut world = World::new();
//         assert!(!world.add_component_storage::<HealthComponent>());
//     }

//     #[test]
//     fn add_component_storage_returns_true_when_already_registered() {
//         let mut world = World::new();
//         world.add_component_storage::<HealthComponent>();
//         assert!(world.add_component_storage::<HealthComponent>());
//     }

//     #[test]
//     fn add_component_storage_is_independent_per_type() {
//         let mut world = World::new();
//         assert!(!world.add_component_storage::<HealthComponent>());
//         // A different type should still be "new".
//         assert!(!world.add_component_storage::<PositionComponent>());
//         // Now both are registered.
//         assert!(world.add_component_storage::<HealthComponent>());
//         assert!(world.add_component_storage::<PositionComponent>());
//     }

//     // invalid entity operations

//     #[test]
//     fn add_component_to_invalid_entity_is_error() {
//         let mut world = World::new();
//         let fake = EntityId(42);
//         assert!(world.add_component(fake, HealthComponent(10)).is_err());
//     }

//     #[test]
//     fn get_component_from_invalid_entity_is_error() {
//         let world = World::new();
//         let fake = EntityId(42);
//         assert!(world.get_entity_component::<HealthComponent>(fake).is_err());
//     }

//     #[test]
//     fn get_component_mut_from_invalid_entity_is_error() {
//         let mut world = World::new();
//         let fake = EntityId(42);
//         assert!(world.get_entity_component_mut::<HealthComponent>(fake).is_err());
//     }

//     #[test]
//     fn remove_component_from_invalid_entity_is_error() {
//         let mut world = World::new();
//         let fake = EntityId(42);
//         assert!(world.remove_component::<HealthComponent>(fake).is_err());
//     }

//     // unregistered component type

//     #[test]
//     fn get_component_never_registered_is_error() {
//         let mut world = World::new();
//         let id = world.create_entity();
//         // HealthComponent storage was never created for this world.
//         assert!(world.get_entity_component::<HealthComponent>(id).is_err());
//     }

//     #[test]
//     fn remove_component_never_registered_is_error() {
//         let mut world = World::new();
//         let id = world.create_entity();
//         assert!(world.remove_component::<HealthComponent>(id).is_err());
//     }

//     // remove then re-add

//     #[test]
//     fn remove_and_readd_component_works() {
//         let mut world = World::new();
//         let id = world.create_entity();

//         world.add_component(id, HealthComponent(50)).unwrap();
//         let removed = world.remove_component::<HealthComponent>(id).unwrap();
//         assert_eq!(50, removed.0);

//         // After removal the slot is gone; re-adding should succeed.
//         world.add_component(id, HealthComponent(99)).unwrap();
//         assert_eq!(99, world.get_entity_component::<HealthComponent>(id).unwrap().0);
//     }

//     #[test]
//     fn remove_and_readd_preserves_other_components() {
//         let mut world = World::new();
//         let id = world.create_entity();

//         world.add_component(id, HealthComponent(10)).unwrap();
//         world.add_component(id, PositionComponent([3, 3, 3])).unwrap();

//         world.remove_component::<HealthComponent>(id).unwrap();

//         // PositionComponent must be untouched.
//         assert_eq!(
//             [3, 3, 3],
//             world.get_entity_component::<PositionComponent>(id).unwrap().0
//         );

//         world.add_component(id, HealthComponent(20)).unwrap();
//         assert_eq!(20, world.get_entity_component::<HealthComponent>(id).unwrap().0);
//     }

//     // disjoint component sets

//     #[test]
//     fn entities_with_disjoint_components_do_not_interfere() {
//         let mut world = World::new();

//         let player = world.create_entity();
//         let enemy = world.create_entity();

//         world.add_component(player, PlayerTag).unwrap();
//         world.add_component(enemy, EnemyTag).unwrap();
//         world.add_component(player, HealthComponent(100)).unwrap();
//         world.add_component(enemy, PositionComponent([5, 0, 0])).unwrap();

//         // Player has no PositionComponent.
//         assert!(world.get_entity_component::<PositionComponent>(player).is_err());
//         // Enemy has no HealthComponent.
//         assert!(world.get_entity_component::<HealthComponent>(enemy).is_err());
//         // Each entity still has its own data intact.
//         assert_eq!(100, world.get_entity_component::<HealthComponent>(player).unwrap().0);
//         assert_eq!([5, 0, 0], world.get_entity_component::<PositionComponent>(enemy).unwrap().0);
//     }

//     // get_entity_component_mut

//     #[test]
//     fn mutation_via_get_mut_is_visible_on_next_get() {
//         let mut world = World::new();
//         let id = world.create_entity();

//         world.add_component(id, PositionComponent([0, 0, 0])).unwrap();

//         world.get_entity_component_mut::<PositionComponent>(id).unwrap().0 = [7, 8, 9];

//         assert_eq!(
//             [7, 8, 9],
//             world.get_entity_component::<PositionComponent>(id).unwrap().0
//         );
//     }

//     #[test]
//     fn mutation_on_one_entity_does_not_affect_another() {
//         let mut world = World::new();
//         let a = world.create_entity();
//         let b = world.create_entity();

//         world.add_component(a, HealthComponent(10)).unwrap();
//         world.add_component(b, HealthComponent(20)).unwrap();

//         world.get_entity_component_mut::<HealthComponent>(a).unwrap().0 = 99;

//         assert_eq!(99, world.get_entity_component::<HealthComponent>(a).unwrap().0);
//         assert_eq!(20, world.get_entity_component::<HealthComponent>(b).unwrap().0);
//     }
// }
