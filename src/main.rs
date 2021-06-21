use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(add_squirrels.system())
        .add_system(hello_world_system.system())
        .add_system(greet_squirrels.system())
        .run();
}

fn hello_world_system() {
    println!("Hello World!");
}

fn greet_squirrels(query: Query<&Name, With<Squirrel>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}

struct Squirrel;

struct Name(String);

fn add_squirrels(mut commands: Commands) {
    commands
        .spawn()
        .insert(Squirrel)
        .insert(Name("Ingrid".to_string()));
    commands
        .spawn()
        .insert(Squirrel)
        .insert(Name("Ana".to_string()));
    commands
        .spawn()
        .insert(Squirrel)
        .insert(Name("Shep".to_string()));
}
