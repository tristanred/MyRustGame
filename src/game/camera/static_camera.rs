use bevy::prelude::*;

use crate::game::character::player::GamePlayer;

#[derive(Component)]
pub struct PlayerFollowCamera;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PlayerFollowCamera));
}

pub fn print_camera_dims(q: Query<&Camera>) {
    // if let Ok(camera) = q.get_single() {
    //     // the size of the area being rendered to
    //     let view_dimensions = camera.logical_viewport_size().unwrap();
    //     debug!("VP Size {:?}", view_dimensions);

    //     // the top-left and bottom-right coordinates
    //     let rec = camera.logical_viewport_rect().unwrap();

    //     debug!("VP Rect {:?}", rec);
    // }
}

pub fn execute_follow_camera(
    mut q: Query<&mut Transform, (With<PlayerFollowCamera>, Without<GamePlayer>)>,
    player: Query<&Transform, With<GamePlayer>>,
) {
    if let (Ok(mut camera), Ok(player)) = (q.get_single_mut(), player.get_single()) {
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
        camera.translation.z = 1.0; // Must stay at 1
    }
}
