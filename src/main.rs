use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// Range at which squirrels show flocking behaviour
const FLOCKING_RANGE: f32 = 200.0;

// How strongly aspects of flocking behaviour are expressed
const CENTERING_FACTOR: f32 = 0.005;
const ALIGNMENT_FACTOR: f32 = 0.05;
const SEPARATION_FACTOR: f32 = 0.05;
const SEPARATION_DISTANCE: f32 = 30.0;
const REGULATION_FACTOR: f32 = 0.005;

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
                    .with_system(center_squirrels.system())
                    .with_system(align_squirrels.system())
                    .with_system(separate_squirrels.system())
                    .with_system(regulate_velocities.system())
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
    id: u32,
}

fn setup(mut commands: Commands, bounds: ResMut<Bounds>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut rng = rand::thread_rng();

    for i in 0..100 {
        // this maths ensures we never get initialise a zero heading
        let y_heading: f32 = rng.gen_range(-1.0..1.0);

        let x_dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let x_heading = (1.0 - y_heading.abs()) * x_dir;

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::RegularPolygon {
                    sides: 3,
                    feature: shapes::RegularPolygonFeature::Radius(10.0),
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
                    * Vec3::new(x_heading, y_heading, 0.0).normalize(),
                id: i,
            });
    }
}

// pull squirrels towards the centre of their local flock
fn center_squirrels(mut query: Query<(&mut Squirrel, &Transform)>) {
    let mut locations: Vec<Vec3> = Vec::new();
    for (_, transform) in query.iter_mut() {
        locations.push(transform.translation);
    }

    for (mut squirrel, transform) in query.iter_mut() {
        let mut center = Vec3::ZERO;
        let mut num_neighbours = 0;

        for sub_translation in &locations {
            if transform.translation.distance(*sub_translation) < FLOCKING_RANGE {
                center += *sub_translation;
                num_neighbours += 1;
            }
        }

        if num_neighbours >= 2 {
            center.x = center.x / num_neighbours as f32;
            center.y = center.y / num_neighbours as f32;

            squirrel.velocity += (center - transform.translation) * CENTERING_FACTOR;
        }
    }
}

// match velocities with other squirrels in local flock
fn align_squirrels(mut query: Query<(&mut Squirrel, &Transform)>) {
    let mut squirrels: Vec<(Vec3, Vec3)> = Vec::new();
    for (squirrel, transform) in query.iter_mut() {
        squirrels.push((squirrel.velocity, transform.translation))
    }

    for (mut squirrel, transform) in query.iter_mut() {
        let mut avg_velocity = Vec3::ZERO;
        let mut num_neighbours = 0;

        for (sub_velocity, sub_translation) in &squirrels {
            if transform.translation.distance(*sub_translation) < FLOCKING_RANGE {
                avg_velocity += *sub_velocity;
                num_neighbours += 1;
            }
        }

        if num_neighbours >= 2 {
            avg_velocity.x = avg_velocity.x / num_neighbours as f32;
            avg_velocity.y = avg_velocity.y / num_neighbours as f32;

            let sqv = squirrel.velocity.clone();
            squirrel.velocity += (avg_velocity - sqv) * ALIGNMENT_FACTOR;
        }
    }
}

// avoid crowding nearby squirrels
fn separate_squirrels(mut query: Query<(&mut Squirrel, &Transform)>) {
    let mut squirrels: Vec<(u32, Vec3)> = Vec::new();
    for (squirrel, transform) in query.iter_mut() {
        squirrels.push((squirrel.id, transform.translation))
    }

    for (mut squirrel, transform) in query.iter_mut() {
        let mut delta_v = Vec3::ZERO;

        for (sub_id, sub_translation) in &squirrels {
            if transform.translation.distance(*sub_translation) < SEPARATION_DISTANCE
                && squirrel.id != *sub_id
            {
                delta_v += transform.translation - *sub_translation
            }
        }

        squirrel.velocity += delta_v * SEPARATION_FACTOR;
    }
}

// Keep squirrels from going too fast or slow
fn regulate_velocities(mut query: Query<&mut Squirrel>) {
    for mut squirrel in query.iter_mut() {
        let sqv = squirrel.velocity.clone();

        if squirrel.velocity.length_squared() < 500.0 * 500.0 {
            squirrel.velocity += REGULATION_FACTOR * sqv;
        } else if squirrel.velocity.length_squared() > 800.0 * 800.0 {
            squirrel.velocity -= REGULATION_FACTOR * sqv;
        }
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

        // Set sprite angle according to velocity TODO: there should be a better way...
        let direction = -squirrel.velocity.x.signum(); // compensates for the fact that angle_between finds the smallest angle regarless of direction
        transform.rotation = Quat::from_rotation_z(
            squirrel.velocity.angle_between(Vec3::new(0.0, 1.0, 0.0)) * direction,
        );
    }
}

fn update_bounds(mut bounds: ResMut<Bounds>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    bounds.x = window.width() / 2 as f32;
    bounds.y = window.height() / 2 as f32;
}
