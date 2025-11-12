use rand::random_range;
use tinecs::{
    Component, Master,
    arguments::{Query, QueryMut, With},
};

impl Component for Robot {}
struct Robot;

impl Component for Position {}
struct Position((isize, isize));

impl Component for Velocity {}
struct Velocity((isize, isize));

fn move_robot(pos: QueryMut<Position, With<Robot>>, vel: Query<Velocity, With<Robot>>) {
    for pos in pos {
        let Position((mut x, mut y)) = *pos;
        let Velocity((dx, dy)) = vel.make_singular();
        x += dx;
        y += dy;
        println!("robot is at: ({}, {})", x, y);
    }
}

fn control_robot(vel: QueryMut<Velocity, With<Robot>>) {
    for mut vel in vel {
        *vel = Velocity((random_range(-1..=1) as isize, random_range(-1..=1) as isize));
    }
}

fn main() {
    let mut ecs = Master::default();
    let robot = ecs.create_entity();
    ecs.add_component(robot, Robot);
    ecs.add_component(robot, Position((0, 0)));
    ecs.add_component(robot, Velocity((0, 0)));
    ecs.add_system(move_robot);
    ecs.add_system(control_robot);
    loop {
        ecs.run();
    }
}
