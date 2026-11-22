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

struct PlayerTag;
struct EnemyTag;

fn check_enemy_and_player(lol: (&Glyph, &Health)) {
    println!("{:?} {:?}", lol.0, lol.1);
}

fn main() -> anyhow::Result<()> {
    let mut world = ecs::World::new();
    let e = world.spawn();
    _ = world.insert_component(e, Health(32.0));
    _ = world.insert_component(e, Glyph('a'));
    let e2 = world.spawn();
    _ = world.insert_component(e2, Health(33.0));
    _ = world.insert_component(e2, Glyph('b'));

    world.run_system::<_, ecs::Query<(&Glyph, &Health)>>(check_enemy_and_player, e);

    Ok(())
}
