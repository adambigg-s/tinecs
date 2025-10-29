// basically, this just copies the `Bevy QuickStart` guide

impl Component for NameTag {}
struct NameTag {
    name: String,
}

use tinecs::{Component, Master, Query, QueryMut};

fn main() {
    let mut master = Master::default();
    let person1 = master.create_entity();
    let person2 = master.create_entity();
    master.add_component(person1, NameTag { name: "john".to_string() });
    master.add_component(person2, NameTag { name: "quan".to_string() });

    master.add_system(greetings);
    master.add_system(greet_people);
    master.run();

    master.add_system(change_names);
    master.run();
}

fn greetings() {
    println!("hello!");
}

fn greet_people(query: Query<NameTag>) {
    for name in query.into_iter() {
        println!("why, hello {}", name.name);
    }
}

fn change_names(query: QueryMut<NameTag>) {
    for mut name in query.into_iter() {
        if name.name == "john" {
            name.name = "tao".to_string();
        }
        if name.name == "quan" {
            name.name = "adam".to_string();
        }
    }
}
