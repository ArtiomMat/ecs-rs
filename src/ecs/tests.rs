use super::*; // Import items from the parent module

struct PositionComponent([i32; 3]);

struct HealthComponent(i32);

struct PlayerTag;
/// Ensures entities don't get mixed when removing and adding components.
#[test]
fn multiple_entities_not_mixed() {
    let mut world = World::new();

    let a = world.create_entity();
    let b = world.create_entity();
    let c = world.create_entity();

    world
        .add_entity_component(c, PositionComponent([2, 2, 2]))
        .unwrap();
    world
        .add_entity_component(a, PositionComponent([0, 0, 0]))
        .unwrap();
    world
        .add_entity_component(b, PositionComponent([1, 1, 1]))
        .unwrap();

    world.add_entity_component(b, PlayerTag).unwrap();
    world.add_entity_component(a, PlayerTag).unwrap();

    world.add_entity_component(b, HealthComponent(1)).unwrap();
    world.add_entity_component(c, HealthComponent(2)).unwrap();
    world.add_entity_component(a, HealthComponent(0)).unwrap();

    // First let's remove the last component that was added in HealtComponent
    assert_eq!(
        0,
        world
            .remove_entity_component::<HealthComponent>(a)
            .unwrap()
            .0
    );
    // Then remove the first that was added.
    assert_eq!(
        1,
        world
            .remove_entity_component::<HealthComponent>(b)
            .unwrap()
            .0
    );
    // Then remove the last remaining.
    assert_eq!(
        2,
        world
            .remove_entity_component::<HealthComponent>(c)
            .unwrap()
            .0
    );

    // First let's remove the first component that was added in HealtComponent
    assert_eq!(
        [2, 2, 2],
        world
            .remove_entity_component::<PositionComponent>(c)
            .unwrap()
            .0
    );
    // Then the second that was added which is now the first.
    assert_eq!(
        [0, 0, 0],
        world
            .remove_entity_component::<PositionComponent>(a)
            .unwrap()
            .0
    );
    // Then remove the last remaining.
    assert_eq!(
        [1, 1, 1],
        world
            .remove_entity_component::<PositionComponent>(b)
            .unwrap()
            .0
    );
}

#[test]
fn single_entity_component_sanity() {
    let mut world = World::new();

    let player_id = world.create_entity();
    world
        .add_entity_component(player_id, HealthComponent(100))
        .unwrap();
    world
        .add_entity_component(player_id, PositionComponent([1, 2, 3]))
        .unwrap();
    world.add_entity_component(player_id, PlayerTag).unwrap();

    assert!(
        world.add_entity_component(player_id, PlayerTag).is_err(),
        "Adding again is an error."
    );

    assert_eq!(
        100,
        world
            .get_entity_component::<HealthComponent>(player_id)
            .unwrap()
            .0
    );

    world
        .get_entity_component_mut::<HealthComponent>(player_id)
        .unwrap()
        .0 = 67;

    assert_eq!(
        67,
        world
            .get_entity_component::<HealthComponent>(player_id)
            .unwrap()
            .0
    );

    assert_eq!(
        67,
        world
            .remove_entity_component::<HealthComponent>(player_id)
            .unwrap()
            .0
    );

    assert!(
        world
            .remove_entity_component::<HealthComponent>(player_id)
            .is_err()
    );

    println!(
        "{}",
        world
            .remove_entity_component::<HealthComponent>(player_id)
            .err()
            .unwrap()
    );

    assert!(
        world
            .get_entity_component::<HealthComponent>(player_id)
            .is_err()
    );
    assert_eq!(
        [1, 2, 3],
        world
            .get_entity_component::<PositionComponent>(player_id)
            .unwrap()
            .0
    );
}
