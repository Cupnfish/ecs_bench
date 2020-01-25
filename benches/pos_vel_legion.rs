#![feature(test)]
extern crate test;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_VEL};
use legion::prelude::*;
use test::Bencher;

fn build() -> World {
    let universe = Universe::new();
    let mut world = universe.create_world();

    world.insert(
        (),
        (0..N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );
    world.insert(
        (),
        (0..N_POS - N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 },)),
    );

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
