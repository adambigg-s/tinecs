# tinecs
minimalist entity-component-system generalized framework experiment in Rust

## Example
```rust
use nalgebra::Vector2;
use rand::random_range;
use tinecs::{
    Component, Master,
    arguments::{Query, QueryMut, With},
    master,
};

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

#### Within the 'examples' folder, there are a few others

#### name_calling.rs
Basically a clone of the Bevy quickstart guide, but using tinecs

#### flippy.rs
A dynamical simulation of a 'T'-handle like object spinning on the intermediate axis.

This example is a good illustration of the strengths of ECS in simulation environments.

This is the most contrived example, and uses some external crates like 'nalgebra' and 'euc' for matrix math and rendering.

![alt text](https://github.com/adambigg-s/tinecs/blob/main/examples/media/triaxial.gif)

## Why?
