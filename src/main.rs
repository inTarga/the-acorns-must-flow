use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins) //TODO: maybe split this up?
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        //TODO:
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_squirrels.system())
            .add_system(greet_squirrels.system());
    }
}

struct GreetTimer(Timer);

fn greet_squirrels(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Squirrel>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
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
