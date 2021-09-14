use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod map;
mod player;

pub const METERS_TO_PIXELS: f32 = 12.; // 10px is 1m

pub const WINDOW_WIDTH: f32 = 80.;
pub const WINDOW_HEIGHT: f32 = 60.;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(WindowDescriptor {
            title: "Donut".to_string(),
            width: WINDOW_WIDTH * METERS_TO_PIXELS,
            height: WINDOW_HEIGHT * METERS_TO_PIXELS,
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            scale: METERS_TO_PIXELS,
            gravity: [0., 0.].into(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(setup.system())
        .add_startup_system(map::spawn_ground.system())
        .add_startup_system(player::spawn_player.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(player::player_jump.system())
        .add_system(player::player_move.system())
        .add_system(player::respawn.system())
        .add_system(player::rotate.system())
        .add_system(player::link_physics.system())
        .add_system(player::limit_velocity.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
