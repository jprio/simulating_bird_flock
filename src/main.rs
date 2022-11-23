//! Renders a 2D scene containing a single, moving sprite.
use bevy::window::PresentMode;
use bevy::{app::PluginGroupBuilder, prelude::*};
use rand::Rng;

const WIDTH: f32 = 1000.;
const HEIGHT: f32 = 600.;
const FLOCK_SIZE: usize = 15;
const MAX_SPEED: f32 = 200.;
const PERCEPTION_LIM: f32 = 200.;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: WIDTH,
                height: HEIGHT,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        })) //.add_startup_system(setup)
        //.add_system(sprite_movement)
        .add_plugin(FlockPlugin)
        .run();
}
#[derive(Component, Default)]
pub struct Position(pub Vec2);
#[derive(Component, Default)]
pub struct Speed(pub Vec2);
#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
struct Boid {}

#[derive(Resource, Debug)]
struct NeighboursTimer(Timer);

#[derive(Component, Debug)]
struct Name(String);
// spawn player system
pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app //.add_system(add_people)
            .add_startup_system(setup)
            //.add_system(display_boids)
            .add_system(sprite_movement)
            .add_system(boid_acceleration)
            .add_system(boid_steering)
            .insert_resource(NeighboursTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        //.add_system(move_boids)
        ;
    }
}

impl PluginGroup for FlockPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(FlockPlugin)
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for i in 1..FLOCK_SIZE {
        let mut rng = rand::thread_rng();
        let position = Vec2::new(
            rng.gen_range(-WIDTH / 2.0..WIDTH / 2.0),
            rng.gen_range(0.0..HEIGHT / 2.0),
        );
        let acceleration = Vec2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
        let speed = Vec2::new(
            rng.gen_range(-MAX_SPEED..MAX_SPEED),
            rng.gen_range(-MAX_SPEED..MAX_SPEED),
        );

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("branding/icon.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.)
                    .with_scale(Vec3::new(0.15, 0.15, 0.)),
                ..default()
            },
            Name(format!("{}-{}", "hello world", i)),
            Speed { 0: speed },
            Acceleration { 0: acceleration },
            Boid {},
        ));
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Transform, &Boid, &Speed)>) {
    for (mut transform, boid, speed) in &mut sprite_position {
        transform.translation.y += speed.0.y * time.delta_seconds();
        transform.translation.x += speed.0.x * time.delta_seconds();
        //println!("moving {:?}", name);
        transform.rotate(Quat::from_rotation_z(0.01f32));
        if transform.translation.y > HEIGHT / 2. {
            transform.translation.y = -HEIGHT / 2.;
        } else if transform.translation.y < -HEIGHT / 2. {
            transform.translation.y = HEIGHT / 2.;
        };
        if transform.translation.x > WIDTH / 2. {
            transform.translation.x = -WIDTH / 2.;
        } else if transform.translation.x < -WIDTH / 2. {
            transform.translation.x = WIDTH / 2.;
        };
    }
}

fn boid_acceleration(time: Res<Time>, mut boids: Query<(&mut Acceleration, &mut Speed)>) {
    let mut rng = rand::thread_rng();

    for (mut acceleration, mut speed) in &mut boids {
        if speed.0.length() < 200. {
            speed.0.x += acceleration.0.x;
            speed.0.x += acceleration.0.x;
        } else {
            speed.0 = Vec2::new(
                rng.gen_range(-WIDTH / 2.0..WIDTH / 2.0),
                rng.gen_range(0.0..HEIGHT / 2.0),
            )
        }
    }
}
fn boid_steering(
    time: Res<Time>,
    mut timer: ResMut<NeighboursTimer>,
    mut query: Query<(&mut Boid, &Transform, &Speed, Entity)>,
    inner_query: Query<(&Transform, &Speed, &Acceleration, Entity), With<Boid>>, // This query requires the Boid component but doesn't borrow it
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello {:?}!", "h".to_string());
        query.for_each_mut(|(mut boid, transform_a, mut velocity_a, entity_a)| {
            let mut avg_speed = Vec2::new(0., 0.);
            let mut contributeurs: f32 = 0.;

            inner_query.for_each(|(transform_b, velocity_b, acceleration, entity_b)| {
                if !(entity_a == entity_b) {
                    let pos_a = transform_a.translation.truncate();
                    let pos_b = transform_b.translation.truncate();
                    let distance = pos_a.distance(pos_b);
                    println!("{}", distance.to_string());
                    if distance < PERCEPTION_LIM {
                        println!("{:?} est proche de moi", entity_b);
                        contributeurs += 1.;
                        avg_speed += velocity_b.0;
                    }
                }
            });

            //velocity_a.0 =
            //  Vec2::new(avg_speed.x / contributeurs, avg_speed.y / contributeurs) + velocity_a.0;
        });

        //for (boid, name) in boids.iter_mut() {
        //for other_boid in other_boids.iter() {}
        //let mut iter = boids.iter_combinations_mut();
        //while let Some([b1, b2]) = iter.fetch_next() {}
        //}
    }
}
