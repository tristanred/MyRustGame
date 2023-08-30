use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // Startup/Setup Systems
        .add_systems(Startup, game::camera::static_camera::setup)
        .add_systems(Startup, game::background::floor::setup)
        .add_systems(Startup, game::background::walls::setup_walls)
        .add_systems(Startup, game::character::player::setup_player)
        .add_systems(PostStartup, game::layout::game_layout::setup_dimensions)
        // Update systems
        //.add_systems(Update, game::camera::static_camera::keyboard_input_system)
        .add_systems(Update, game::character::player::player_handle_jump)
        .add_systems(Update, game::character::player::player_handle_movement)
        .add_systems(Update, game::camera::static_camera::execute_follow_camera)
        .run();
}
