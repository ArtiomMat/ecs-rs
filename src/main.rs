use std::error::Error;

mod ecs;

struct Health(f32);
struct Transform {
    position: [f32; 3],
    rotation: [f32; 3],
}
#[derive(Clone, Copy)]
struct Glyph(char);

struct PlayerTag;
struct EnemyTag;

struct Animal {

}

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = ecs::World::new();

    world.add_component_storage::<Health>();
    world.add_component_storage::<Transform>();
    world.add_component_storage::<Glyph>();
    world.add_component_storage::<PlayerTag>();
    world.add_component_storage::<EnemyTag>();

    let player = world.create_entity();
    world.add_component(player, PlayerTag)?;
    world.add_component(player, Health(3.0))?;
    world.add_component(
        player,
        Transform {
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0],
        },
    )?;

    let enemy_a = world.create_entity();
    world.add_component(enemy_a, EnemyTag)?;
    world.add_component(enemy_a, Health(1.0))?;
    world.add_component(enemy_a, Glyph('a'))?;

    let enemy_b = world.create_entity();
    world.add_component(enemy_b, EnemyTag)?;
    world.add_component(enemy_b, Health(2.0))?;
    world.add_component(enemy_b, Glyph('b'))?;

    world.query::<(Option<ecs::Read<Glyph>>, ecs::Read<Health>, ecs::With<Transform>)>(|(glyph, hp, _)| {
        println!("Enemy {} {}", glyph.map(|x| x.clone()).unwrap_or(Glyph('!')).0, hp.0);
    });

    Ok(())
}
