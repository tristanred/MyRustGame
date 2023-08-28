use bevy::prelude::*;

#[derive(Component)]
pub struct StaticCamera;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), StaticCamera));

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

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut StaticCamera, &mut Transform)>,
) {
    for c in &mut query {
        let (_, _, mut transform) = c;
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 1.0;
        }
    }
}
