use bevy::{core::FixedTimestep, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;
const X_BOUND: f32 = 800.0; //TODO: figure out what the actual screen bounds are,
const Y_BOUND: f32 = 400.0; // and maybe do it dynamically?

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

fn move_squirrels(mut query: Query<(&mut Squirrel, &mut Transform)>) {
    if let Ok((mut squirrel, mut transform)) = query.single_mut() {
        // Reverse velocity component if going out of frame
        if (transform.translation.x > X_BOUND && squirrel.velocity.x > 0.0)
            || (transform.translation.x < -X_BOUND && squirrel.velocity.x < 0.0)
        {
            squirrel.velocity.x = -squirrel.velocity.x;
        } else if (transform.translation.y > Y_BOUND && squirrel.velocity.y > 0.0)
            || (transform.translation.y < -Y_BOUND && squirrel.velocity.y < 0.0)
        {
            squirrel.velocity.y = -squirrel.velocity.y;
        }

        // Movement for this time step
        transform.translation += squirrel.velocity * TIME_STEP;
    }
}
