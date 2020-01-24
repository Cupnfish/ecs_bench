#![feature(test)]
extern crate test;

use test::Bencher;
use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};
use legion::prelude::*;

fn build() -> World {
    let universe = Universe::new();
    let mut world = universe.create_world();

    for i in 0..N_POS {
//        let entity = world.insert((), Some((Position { x: 0.0, y: 0.0 },)))[0];
//        if i % N_POS_PER_VEL == 0 {
//            world.add_component(entity, Velocity { dx: 0.0, dy: 0.0 });
//        }

        if i % N_POS_PER_VEL == 0 {
            world.insert((), Some((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 })));
        } else {
            world.insert((), Some((Position { x: 0.0, y: 0.0 },)));
        }
    }
    world
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();
    let query = <(Write<Position>, Read<Velocity>)>::query();

    b.iter(|| {
        for (mut pos, vel) in query.iter(&mut world) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }

    });
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}
