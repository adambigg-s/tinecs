use tinecs::{Component, Query, master};

fn main() {
    let mut master = master();
    let entity = master.create_entity();
    master.add_component(entity, Position { x: 33, y: 99 });
    master.add_system(say_hello);
    master.add_system(say_position);
    master.run_systems();
}

impl Component for Position {}
struct Position {
    x: i32,
    y: i32,
}

fn say_hello() {
    println!("hello called");
}

fn say_position(query: Query<Position>) {
    for pos in query {
        println!("position: ({}, {})", pos.x, pos.y);
    }
}
