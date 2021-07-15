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
        app.insert_resource(Bounds { x: 0.0, y: 0.0 })
            .add_startup_system(setup.system())
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

struct MainCamera;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bounds: ResMut<Bounds>,
    windows: Res<Windows>,
) {
    update_bounds(bounds, windows);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
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

fn move_squirrels(mut query: Query<(&mut Squirrel, &mut Transform)>, bounds: Res<Bounds>) {
    if let Ok((mut squirrel, mut transform)) = query.single_mut() {
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
    }
}

fn update_bounds(mut bounds: ResMut<Bounds>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    bounds.x = window.width() / 2 as f32;
    bounds.y = window.height() / 2 as f32;
}
