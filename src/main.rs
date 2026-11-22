use std::error::Error;

use crate::ecs::EntityId;

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

/*
struct DamageSystemLocal {
    tick: usize,
}

fn move_enemies_system(
    enemies: Query<(&Glyph, &mut Transform), With<EnemyTag>>,
    players: Query<(&Glyph, &Transform), (With<PlayerTag>, Without<Invisible>)>,
    resource: Resource<Option<&mut Dungeon>>,
    local: Local<DamageSystemLocal>,
) {

}
*/

fn move_enemies_system(
    enemies: ecs::Query<(&Glyph, &mut Transform), ecs::With<EnemyTag>>,
    // players: ecs::Query<(&Glyph, &Transform), ecs::With<PlayerTag>>,
) {

}

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = ecs::World::new();

    /*
    world
        .begin_frame()
        .add_system(move_enemies_system)
        .execute();
    */

    let player = world.spawn();
    world.add_component(player, PlayerTag)?;
    world.add_component(player, Health(3.0))?;
    world.add_component(
        player,
        Transform {
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0],
        },
    )?;

    let enemy_a = world.spawn();
    world.add_component(enemy_a, EnemyTag)?;
    world.add_component(enemy_a, Health(1.0))?;
    world.add_component(enemy_a, Glyph('a'))?;

    let enemy_b = world.spawn();
    world.add_component(enemy_b, EnemyTag)?;
    world.add_component(enemy_b, Health(2.0))?;
    world.add_component(enemy_b, Glyph('b'))?;

    // world.query::<((ecs::Read<Health>, Option<ecs::Read<Glyph>>), (Option<ecs::Read<Glyph>>, ecs::With<Transform>))>(|((hp, glyph), _)| {
    //     println!("Enemy {} {}", glyph.map(|x| x.clone()).unwrap_or(Glyph('!')).0, hp.0);
    // });

    world.run_system(move_enemies_system);


    // let enemies_query = ecs::Query::<(EntityId, &Health, Option<&Glyph>), (Option<&Glyph>, ecs::Without<Transform>)>::new(&world);
    // for (entity_id, health, glyph) in enemies_query.iter() {
    //     println!("Enemy({}): {} {}", entity_id, glyph.map(|x| x.clone()).unwrap_or(Glyph('!')).0, health.0);
    // }

    Ok(())
}
