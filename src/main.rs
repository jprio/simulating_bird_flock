//! Renders a 2D scene containing a single, moving sprite.
use bevy::window::PresentMode;
use bevy::{app::PluginGroupBuilder, prelude::*};
use rand::Rng;
const WIDTH: f32 = 500.;
const HEIGHT: f32 = 300.;
const FLOCK_SIZE: u8 = 15;
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

// spawn player system
pub struct FlockPlugin;

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app //.add_system(add_people)
            .add_startup_system(setup)
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

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for i in 1..FLOCK_SIZE {
        let mut rng = rand::thread_rng();
        let position = Vec2::new(
            rng.gen_range(-WIDTH / 2.0..WIDTH / 2.0),
            rng.gen_range(0.0..HEIGHT / 2.0),
        );

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("branding/icon.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.)
                    .with_scale(Vec3::new(0.2, 0.2, 0.)),
                ..default()
            },
            Direction::Up,
        ));
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }
        transform.rotate(Quat::from_rotation_z(0.01f32));
        if transform.translation.y > HEIGHT / 2. {
            *logo = Direction::Down;
        } else if transform.translation.y < -HEIGHT / 2. {
            *logo = Direction::Up;
        }
    }
}
