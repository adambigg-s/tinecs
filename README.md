# tinecs
minimalist entity-component-system generalized framework experiment in Rust

## Examples

### robot.rs
```rust
impl Component for Robot {}
struct Robot;

impl Component for Position {}
struct Position(Vector2<isize>);

impl Component for Velocity {}
struct Velocity(Vector2<isize>);

fn move_robot(pos: QueryMut<Position, With<Robot>>, vel: Query<Velocity, With<Robot>>) {
    let Velocity(vel) = vel.make_singular();
    for mut pos in pos {
        let Position(old_pos) = *pos;
        *pos = Position(old_pos + vel);
    }
}

fn control_robot(vel: QueryMut<Velocity, With<Robot>>) {
    for mut vel in vel {
        *vel = Velocity(Vector2::new(random_range(-1..=1) as isize, random_range(-1..=1) as isize));
    }
}

fn report_robot(robot: Query<Position, With<Robot>>) {
    let Position(pos) = robot.make_singular();
    println!("robot is at {{{:?}}}", pos);
}

fn main() {
    let mut ecs = master();
    let robot = ecs.create_entity();
    ecs.add_component(robot, Robot);
    ecs.add_component(robot, Position(Vector2::zeros()));
    ecs.add_component(robot, Velocity(Vector2::zeros()));

    ecs.add_system(move_robot);
    ecs.add_system(control_robot);
    ecs.add_system(report_robot);

    loop {
        ecs.run();
    }
}
```

Displays the location of a 'robot' running around randomly

### name_calling.rs
Basically a one-to-one clone of the Bevy quickstart guide, but using tinecs

### flippy.rs
A dynamical simulation of a 'T'-handle like object spinning on the intermediate axis

This example is a good illustration of the strengths of ECS in simulation environments

This is the most contrived example, and uses some external crates like 'nalgebra' and 'euc' for matrix math and rendering

![alt text](https://github.com/adambigg-s/tinecs/blob/main/examples/media/triaxial.gif)

## Why?

## Performance Benchmarks
When running **fib_speed.rs**, we find that the ECS solution is around 34.3x slower. This sounds really bad, but when analyzing the optimized assembly, we find the total Fibonacci function is literally 5 instructions total -- so, this is basically the lowest overhead function possible.

In other words, this is a fairly bad benchmark for real applications, as (35) x (basically zero) = (still basically zero), but is really good at giving an idea of how much overhead the ECS database lookup adds, when compared to directly altering a value.

Basically, these scalar operations are overwhelmingly dominated by the HashMap queries.

When running **matmul_speed.rs**, we find that the ECS is around 3.7x slower, which is actually still not great. But, in the grand scheme of things a 4x4 matrix similarity transformation is probably close to the smallest single system that would be implemented in a game/simulation. So, as complexity grows this number would approach 1x speed pretty quickly.

## Fundamental Types

## Resources
- https://github.com/bevyengine/bevy.git
- https://github.com/tokio-rs/axum.git
- https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch.git

