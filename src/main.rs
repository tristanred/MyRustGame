use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, game::camera::static_camera::setup)
        .add_systems(Startup, game::background::floor::setup)
        .add_systems(Update, game::camera::static_camera::keyboard_input_system)
        .add_systems(PostStartup, game::camera::static_camera::print_camera_dims)
        .run();
}
