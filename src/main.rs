use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, game::camera::static_camera::setup)
        .add_systems(Update, game::camera::static_camera::keyboard_input_system)
        .run();
}
