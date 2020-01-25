# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [legion]        | 1,353 µs/iter (+/- 50)        | 1 µs/iter (+/- 0)        | {parallel_build_legion}        | {parallel_update_legion}
 [tiny_ecs]      | 266 µs/iter (+/- 24)      | 9 µs/iter (+/- 0)      | {parallel_build_tiny_ecs}      | {parallel_update_tiny_ecs}
 [hecs]          | 383 µs/iter (+/- 20)          | 4 µs/iter (+/- 0)          | {parallel_build_hecs}          | {parallel_update_hecs}
 [specs]         | 311 µs/iter (+/- 15)         | 3 µs/iter (+/- 0)         | {parallel_build_specs}         | {parallel_update_specs}
 [froggy]        | 304 µs/iter (+/- 14)        | 9 µs/iter (+/- 0)        | {parallel_build_froggy}        | {parallel_update_froggy}
 [constellation] | 222 µs/iter (+/- 15) | 6 µs/iter (+/- 0) | {parallel_build_constellation} | {parallel_update_constellation}

[legion]: https://github.com/jaynus/legion
[constellation]: https://github.com/TomGillen/constellation/
[hecs]: https://github.com/Ralith/hecs/
[froggy]: https://github.com/kvark/froggy
[specs]: https://github.com/slide-rs/specs
[tiny_ecs]: https://gitlab.com/flukejones/tiny_ecs/


Visualization of benchmarks, smaller is better.
![update benchmarks graph](./graph/update.png)
![build benchmarks graph](./graph/build.png)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
 * systems could be run in parallel

## Notes
 * the benchmarks explore a limited subset of ECS use-cases and do not necessarily reflect the peformance of large-scale applications
 * [froggy](https://github.com/kvark/froggy) is technically not an ECS, but a Component Graph System (CGS)
