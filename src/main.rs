use std::error::Error;

mod ecs;

struct Health(f32);
struct Transform {
    position: [f32; 3],
    rotation: [f32; 3],
}
struct Glyph(char);

struct PlayerTag;
struct EnemyTag;

struct Animal {

}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut world = ecs::World::new();

    // world.add_component_storage::<Health>();
    // world.add_component_storage::<Transform>();
    // world.add_component_storage::<Glyph>();
    // world.add_component_storage::<PlayerTag>();
    // world.add_component_storage::<EnemyTag>();

    // let player = world.create_entity();
    // world.add_entity_component(player, PlayerTag)?;
    // world.add_entity_component(player, Health(1.0))?;
    // world.add_entity_component(
    //     player,
    //     Transform {
    //         position: [1.0, 2.0, 3.0],
    //         rotation: [0.0, 0.0, 0.0],
    //     },
    // )?;

    // for i in 0..10 {
    //     let enemy = world.create_entity();
    //     world.add_entity_component(enemy, EnemyTag)?;
    //     world.add_entity_component(enemy, Health(1.0))?;
    //     world.add_entity_component(
    //         enemy,
    //         Transform {
    //             position: [1.0, 2.0, 3.0],
    //             rotation: [0.0, 0.0, 0.0],
    //         },
    //     )?;
    // }    

    // let query = Query::<(&Health, &Transform)>::new(&world);
    // let query = QueryMut::<(&mut Health, &Transform)>::new(&mut world);
    // let filtered_query = FilterQuery::<(&Net, &Transform), (With<PlayerTag>, Dirty<Transform>)>::new(&mut world);
    // for (&mut health, &mut transform) in query {
    //     transform.position[0] += health.0;
    // }
    
    // world.query::<(&mut Health, &Transform)>()

    Ok(())
}
