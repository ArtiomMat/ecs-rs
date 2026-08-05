use std::cell::{Ref, RefMut};

use crate::{inspect::Inspect, system_runner::run_system, type_map::TypeMap};

mod type_map;
mod inspect;
mod system_runner;

#[derive(Debug)]
struct Health(f32);

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32
}

fn main() {
    let mut map = TypeMap::new();
    map.insert(Health(100.0));
    map.insert(Position { x: 1.0, y: 2.0, z: 3.0 });

    run_system::<(RefMut<Health>, Ref<Position>)>(&map, |(mut hp, pos)| {
        println!("{:?}, {:?}", hp, pos);
        hp.0 = 50.0;
    });
    
    run_system::<(RefMut<Position>, Ref<Health>)>(&map, |(mut pos, hp)| {
        println!("{:?}, {:?}", hp, pos);
        pos.x += 50.0;
    });

    run_system::<Ref<Position>>(&map, |pos| {
        println!("{:?}", pos);
    });

    // Will not run
    run_system::<(RefMut<Position>, Ref<Position>)>(&map, |(mut a, b)| {
        println!("{:?}, {:?}", a, b);
        a.x += 50.0;
    });

    // Will run
    run_system::<(Ref<Position>, Ref<Position>)>(&map, |(a, b)| {
        println!("{:?}, {:?}", a, b);
    });
}
