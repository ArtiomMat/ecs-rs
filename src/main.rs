mod ecs;
mod mem;

#[derive(Debug, Clone, Copy)]
struct Health(f32);
struct Transform {
    position: [f32; 3],
    rotation: [f32; 3],
}
#[derive(Debug, Clone, Copy)]
struct Glyph(char);

#[derive(Debug, Clone, Copy)]
struct PlayerTag;
#[derive(Debug, Clone, Copy)]
struct EnemyTag;

fn check_enemy_and_player(lol: ecs::Query<(&PlayerTag, &Glyph), ecs::With<Health>>, lol2: ecs::Query<(&EnemyTag, &Glyph), ecs::With<Health>>) {
    for x in lol.iter() {
        println!("PLAYER: {:?} {:?}", x.0, x.1);
    }
    for x in lol2.iter() {
        println!("ENEMY: {:?} {:?}", x.0, x.1);
    }
}

fn main() -> anyhow::Result<()> {
    let mut world = ecs::World::new();
    let e = world.spawn();
    world.insert_component(e, Health(32.0))?;
    world.insert_component(e, EnemyTag)?;
    world.insert_component(e, Glyph('a'))?;
    let e2 = world.spawn();
    world.insert_component(e2, Health(33.0))?;
    world.insert_component(e2, Glyph('b'))?;
    world.insert_component(e2, PlayerTag)?;
    let e3 = world.spawn();
    world.insert_component(e3, Health(34.0))?;
    world.insert_component(e3, Glyph('c'))?;

    world.run_system(check_enemy_and_player);

    Ok(())
}
