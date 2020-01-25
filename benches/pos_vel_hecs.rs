#![feature(test)]
extern crate test;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_VEL};
use hecs::{Bundle, MissingComponent, World};
use test::Bencher;

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

    for _ in 0..N_POS - N_POS_VEL {
        world.spawn(Pos {
            pos: Position { x: 0.0, y: 0.0 },
        });
    }
    for _ in 0..N_POS_VEL {
        world.spawn(PosVel {
            pos: Position { x: 0.0, y: 0.0 },
            vel: Velocity { dx: 1.0, dy: 1.0 },
        });
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
