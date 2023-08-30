use bevy::prelude::*;

use crate::game::character::player::GamePlayer;

#[derive(Component)]
pub struct PlayerFollowCamera;

/// Area where the camera can move. The camera should be able to go a little
/// outside the playable area to let the player say in the center of the screen.
pub const PAN_SPACE: Rect = Rect {
    min: Vec2::new(-180.0, (CAM_HEIGHT / 2.0) - 200.0),
    max: Vec2::new(180.0, 1800.0),
};

#[allow(dead_code)]
pub const CAM_WIDTH: f32 = 720.0;

#[allow(dead_code)]
pub const CAM_HEIGHT: f32 = 1280.0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PlayerFollowCamera));
}

/// System to follow the player with the camera.
/// TODO: The camera should not be able to go past the edges of the map.
pub fn execute_follow_camera(
    mut q: Query<&mut Transform, (With<PlayerFollowCamera>, Without<GamePlayer>)>,
    player: Query<&Transform, With<GamePlayer>>,
) {
    if let (Ok(mut camera), Ok(player)) = (q.get_single_mut(), player.get_single()) {
        let min_x = PAN_SPACE.min.x;
        let max_x = PAN_SPACE.max.x;

        let min_y = PAN_SPACE.min.y;
        let max_y = PAN_SPACE.max.y;

        camera.translation.x = player.translation.x.clamp(min_x, max_x);
        camera.translation.y = player.translation.y.clamp(min_y, max_y);
        camera.translation.z = 1.0; // Must stay at 1
    }
}
