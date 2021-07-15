use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins) //TODO: maybe split this up?
        .add_plugin(TamfPlugin)
        .add_plugin(ShapePlugin)
        .run();
}

pub struct TamfPlugin;

impl Plugin for TamfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Bounds { x: 0.0, y: 0.0 })
            .add_startup_system(update_bounds.system().before("setup"))
            .add_startup_system(setup.system().label("setup"))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(move_squirrels.system())
                    .with_system(update_bounds.system()),
            );
    }
}

struct Bounds {
    x: f32,
    y: f32,
}

struct Squirrel {
    velocity: Vec3,
}

fn setup(mut commands: Commands, bounds: ResMut<Bounds>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::RegularPolygon {
                    sides: 3,
                    feature: shapes::RegularPolygonFeature::Radius(30.0),
                    ..shapes::RegularPolygon::default()
                },
                ShapeColors::new(Color::rgb(rng.gen(), rng.gen(), rng.gen())),
                DrawMode::Fill(FillOptions::default()),
                Transform::from_xyz(
                    rng.gen_range(-bounds.x..bounds.x),
                    rng.gen_range(-bounds.y..bounds.y),
                    0.0,
                ),
            ))
            .insert(Squirrel {
                velocity: rng.gen_range(200.0..600.0)
                    * Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
                        .normalize(),
            });
    }
}

fn move_squirrels(mut query: Query<(&mut Squirrel, &mut Transform)>, bounds: Res<Bounds>) {
    for (mut squirrel, mut transform) in query.iter_mut() {
        // Reverse velocity component if going out of frame
        if (transform.translation.x > bounds.x && squirrel.velocity.x > 0.0)
            || (transform.translation.x < -bounds.x && squirrel.velocity.x < 0.0)
        {
            squirrel.velocity.x = -squirrel.velocity.x;
        } else if (transform.translation.y > bounds.y && squirrel.velocity.y > 0.0)
            || (transform.translation.y < -bounds.y && squirrel.velocity.y < 0.0)
        {
            squirrel.velocity.y = -squirrel.velocity.y;
        }

        // Movement for this time step
        transform.translation += squirrel.velocity * TIME_STEP;

        // Set sprite angle according to velocity
        transform.rotation =
            Quat::from_rotation_z(squirrel.velocity.angle_between(Vec3::new(0.0, 1.0, 0.0)));
    }
}

fn update_bounds(mut bounds: ResMut<Bounds>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    bounds.x = window.width() / 2 as f32;
    bounds.y = window.height() / 2 as f32;
}
