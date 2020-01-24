#![feature(test)]
extern crate test;

use test::Bencher;
use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};
use hecs::{World, Bundle, MissingComponent};

#[derive(Bundle)]
struct PosVel {
    pos: Position,
    vel: Velocity,
}

#[derive(Bundle)]
struct Pos {
    pos: Position,
}

fn build() -> World {
    let mut world = World::new();

    for i in 0..N_POS {
        if i % N_POS_PER_VEL == 0 {
            world.spawn(PosVel {
                pos: Position { x: 0.0, y: 0.0 }, vel: Velocity { dx: 0.0, dy: 0.0 }});
        } else {
            world.spawn(Pos {
                pos: Position { x: 0.0, y: 0.0 }});
        }
    }
    world
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let world = build();

    b.iter(|| {
        for (_, (pos, vel)) in &mut world.query::<(&mut Position, &Velocity)>() {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    });
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}
