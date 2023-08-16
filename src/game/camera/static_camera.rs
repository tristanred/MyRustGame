use bevy::prelude::*;

#[derive(Component)]
pub struct StaticCamera;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), StaticCamera));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("character/Pink_Monster.png"),
        transform: Transform::from_xyz(20.0, 20.0, 0.0),
        ..Default::default()
    });
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
