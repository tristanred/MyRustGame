use bevy::prelude::*;

#[derive(Resource)]
pub struct GameDimensions {
    pub width: u32,
    pub height: u32,
}

pub fn setup_dimensions(mut commands: Commands) {
    commands.insert_resource(GameDimensions {
        width: 720,
        height: 1280,
    });
}
