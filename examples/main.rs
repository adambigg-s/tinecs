// basically, this just copies the `Bevy QuickStart` guide

use tinecs::Component;
use tinecs::Master;
use tinecs::master;

impl Component for NameTag {}
struct NameTag {
    name: String,
}

fn main() {
    let mut master = master();
    let person1 = master.create_entity();
    let person2 = master.create_entity();
    master.add_component(person1, NameTag { name: "john".to_string() });
    master.add_component(person2, NameTag { name: "quan".to_string() });

    say_hello(&master);
    change_names(&mut master);
    say_hello(&master);
}

fn change_names(master: &mut Master) {
    let query = master.query_components_mut::<NameTag>();
    for name in query.into_iter() {
        if name.name == "john" {
            name.name = "tao".to_string();
        }
        if name.name == "quan" {
            name.name = "adam".to_string();
        }
    }
}

fn say_hello(master: &Master) {
    let query = master.query_components::<NameTag>();
    for name in query.into_iter() {
        println!("hello to {}", name.name);
    }
}
