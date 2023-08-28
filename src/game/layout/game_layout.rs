use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
pub struct GameDimensions {
    pub width: u32,
    pub height: u32,
}

pub fn setup_dimensions(mut commands: Commands, mut q: Query<&mut Window, With<PrimaryWindow>>) {
    commands.insert_resource(GameDimensions {
        width: 720,
        height: 1280,
    });

    let mut x = q.single_mut();
    x.resolution.set(720.0, 1280.0);
    x.resizable = false;

    info!(
        "Window Phys Height = {}, Width = {}",
        x.resolution.physical_height(),
        x.resolution.physical_width()
    );
    info!(
        "Window Logical Height = {}, Width = {}",
        x.resolution.height(),
        x.resolution.width()
    );
    info!("Scale factor = {}", x.scale_factor());
}
