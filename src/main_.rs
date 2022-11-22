use bevy::{app::PluginGroupBuilder, prelude::*};
use rand::Rng;
use std::cell::Cell;

const NUM_FLOCK: u8 = 100;

#[derive(Component)]
struct Boid {
    position: Vec2,
    speed: Vec2,
    acceleration: Vec2,
}

#[derive(Component)]
struct Name(String);

fn span_flock(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for i in 1..NUM_FLOCK {
        let mut rng = rand::thread_rng();
        let mut position = Vec2::new(rng.gen_range(-500.0..500.0), rng.gen_range(0.0..1000.0));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("branding/icon.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.)
                    //transform: Transform::from_xyz(100., 0., 0.),
                    .with_scale(Vec3::new(0.1, 0.1, 0.)),
                ..default()
            },
            Boid {
                position: position,
                speed: Vec2::new(0.0, 0.0),
                acceleration: Vec2::new(0.0, 0.0),
            },
            Name(i.to_string()),
        ));
    }
}
#[derive(Resource)]
struct GreetTimer(Timer);
#[derive(Resource)]
struct MoveTimer(Timer);

fn display_boids(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Name, &Boid)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, boid) in query.iter() {
            println!(
                "hello {}, you are at position {},{}!",
                name.0, boid.position.x, boid.position.y
            );
        }
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut sprite_position: Query<(&mut Transform)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut transform) in &mut sprite_position {
            let movey = 1500. * time.delta_seconds();
            println!("transform {} move {}", transform.translation.y, movey);
            transform.translation.y -= movey;
        }
    }
}
fn move_boids(time: Res<Time>, mut timer: ResMut<MoveTimer>, mut query: Query<(&Name, &mut Boid)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, mut boid) in query.iter_mut() {
            if (boid.position.y > 1.0) {
                println!("{}", boid.position.y);
                boid.position.y += -1.0;
                println!("{}", boid.position.y);
            } else {
                boid.position.y = 0.0;
            }
        }
    }
}

// spawn player system
pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(MoveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            //.add_system(add_people)
            .add_startup_system(span_flock)
            //.add_system(display_boids)
            .add_system(sprite_movement)
        //.add_system(move_boids)
        ;
    }
}
impl PluginGroup for FlockPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(FlockPlugin)
    }
}

fn main() {
    App::new()
        //.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        //.insert_resource(MoveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FlockPlugin)
        //.add_system(display_boids)
        .run();
}
