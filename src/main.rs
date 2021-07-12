use bevy::{core::FixedTimestep, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins) //TODO: maybe split this up?
        .add_plugin(TamfPlugin)
        .run();
}

pub struct TamfPlugin;

impl Plugin for TamfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_squirrels.system()),
        );
    }
}

fn move_squirrels(mut query: Query<(&Squirrel, &mut Transform)>) {
    if let Ok((squirrel, mut transform)) = query.single_mut() {
        transform.translation += squirrel.velocity * TIME_STEP;
    }
}

struct Squirrel {
    velocity: Vec3,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Squirrel {
            velocity: 400.0 * Vec3::new(0.5, 0.5, 0.0).normalize(),
        });
}
