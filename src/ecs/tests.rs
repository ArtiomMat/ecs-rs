// #[cfg(test)]
// mod tests {
//     use crate::ecs::{EntityId, Error, Read, With, Without, World, Write};

//     // Sample component types for testing
//     #[derive(Debug, PartialEq, Eq)]
//     struct Position {
//         x: i32,
//         y: i32,
//     }

//     #[derive(Debug, PartialEq, Eq)]
//     struct Velocity {
//         dx: i32,
//         dy: i32,
//     }

//     #[derive(Debug, PartialEq, Eq)]
//     struct Name(String);

//     // ==========================================
//     // Entity Lifecycle Tests
//     // ==========================================

//     #[test]
//     fn test_entity_creation_and_validity() {
//         let mut world = World::new();

//         let e1 = world.create_entity();
//         let e2 = world.create_entity();

//         assert_ne!(e1, e2);
//         assert!(world.is_entity_valid(e1));
//         assert!(world.is_entity_valid(e2));
//         assert!(!world.is_entity_valid(EntityId(9999)));
//     }

//     // ==========================================
//     // Component Management Tests
//     // ==========================================

//     #[test]
//     fn test_add_and_remove_component() {
//         let mut world = World::new();
//         let e1 = world.create_entity();

//         world
//             .add_component(e1, Position { x: 10, y: 20 })
//             .expect("Failed to add component");

//         // Verify removing existing component succeeds
//         let removed = world
//             .remove_component::<Position>(e1)
//             .expect("Failed to remove component");
//         assert_eq!(removed, Position { x: 10, y: 20 });

//         // Removing again should fail
//         assert!(world.remove_component::<Position>(e1).is_err());
//     }

//     #[test]
//     fn test_invalid_entity_operations() {
//         let mut world = World::new();
//         let invalid_e = EntityId(42);

//         assert!(matches!(
//             world.add_component(invalid_e, Position { x: 0, y: 0 }),
//             Err(Error::InvalidEntityId(_))
//         ));

//         assert!(matches!(
//             world.remove_component::<Position>(invalid_e),
//             Err(Error::InvalidEntityId(_))
//         ));
//     }

//     #[test]
//     fn test_add_component_storage_registration() {
//         let mut world = World::new();

//         // First registration returns false (newly added)
//         let existed = world.add_component_storage::<Position>();
//         assert!(!existed);

//         // Second registration returns true (already exists)
//         let existed_again = world.add_component_storage::<Position>();
//         assert!(existed_again);
//     }

//     // ==========================================
//     // Query System Tests
//     // ==========================================

//     #[test]
//     fn test_query_read_and_write() {
//         let mut world = World::new();
//         let e1 = world.create_entity();

//         world.add_component(e1, Position { x: 1, y: 1 }).unwrap();

//         // Test Write: update position
//         world.query::<Write<Position>>(|mut pos| {
//             pos.x += 10;
//             pos.y += 20;
//         });

//         // Test Read: verify updated values
//         let mut read_count = 0;
//         world.query::<Read<Position>>(|pos| {
//             assert_eq!(pos.x, 11);
//             assert_eq!(pos.y, 21);
//             read_count += 1;
//         });
//         assert_eq!(read_count, 1);
//     }

//     #[test]
//     fn test_query_with_and_without_filters() {
//         let mut world = World::new();

//         let e_pos_vel = world.create_entity();
//         let e_pos_only = world.create_entity();
//         let e_vel_only = world.create_entity();

//         world
//             .add_component(e_pos_vel, Position { x: 0, y: 0 })
//             .unwrap();
//         world
//             .add_component(e_pos_vel, Velocity { dx: 1, dy: 1 })
//             .unwrap();

//         world
//             .add_component(e_pos_only, Position { x: 5, y: 5 })
//             .unwrap();

//         world
//             .add_component(e_vel_only, Velocity { dx: -1, dy: -1 })
//             .unwrap();

//         // Query entities with Position but WITHOUT Velocity
//         let mut results = Vec::new();
//         world.query::<(Read<Position>, Without<Velocity>)>(|(pos, _)| {
//             results.push((pos.x, pos.y));
//         });

//         assert_eq!(results.len(), 1);
//         assert_eq!(results[0], (5, 5));

//         // Query entities WITH Velocity
//         let mut vel_count = 0;
//         world.query::<With<Velocity>>(|_| {
//             vel_count += 1;
//         });
//         assert_eq!(vel_count, 2);
//     }

//     #[test]
//     fn test_query_optional_component() {
//         let mut world = World::new();

//         let e1 = world.create_entity();
//         let e2 = world.create_entity();

//         world
//             .add_component(e1, Position { x: 10, y: 10 })
//             .unwrap();
//         world
//             .add_component(e1, Name("Hero".to_string()))
//             .unwrap();

//         world
//             .add_component(e2, Position { x: 20, y: 20 })
//             .unwrap();

//         let mut counts = (0, 0); // (with_name, without_name)

//         world.query::<(Read<Position>, Option<Read<Name>>)>(|(pos, maybe_name)| {
//             if let Some(name) = maybe_name {
//                 assert_eq!(pos.x, 10);
//                 assert_eq!(name.0, "Hero");
//                 counts.0 += 1;
//             } else {
//                 assert_eq!(pos.x, 20);
//                 counts.1 += 1;
//             }
//         });

//         assert_eq!(counts, (1, 1));
//     }

//     #[test]
//     fn test_multi_component_tuple_query() {
//         let mut world = World::new();

//         let e1 = world.create_entity();
//         let e2 = world.create_entity();

//         world.add_component(e1, Position { x: 1, y: 2 }).unwrap();
//         world.add_component(e1, Velocity { dx: 3, dy: 4 }).unwrap();
//         world
//             .add_component(e1, Name("Entity1".to_string()))
//             .unwrap();

//         // e2 misses Velocity, so it shouldn't match a 3-tuple requiring all three
//         world.add_component(e2, Position { x: 10, y: 20 }).unwrap();
//         world
//             .add_component(e2, Name("Entity2".to_string()))
//             .unwrap();

//         let mut matched = 0;
//         world.query::<(Read<Position>, Read<Velocity>, Read<Name>)>((|(pos, vel, name)| {
//             assert_eq!(pos.x, 1);
//             assert_eq!(vel.dx, 3);
//             assert_eq!(name.0, "Entity1");
//             matched += 1;
//         }));

//         assert_eq!(matched, 1);
//     }
// }
