use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct GamePlayer;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
) {
    // Sprite is 32x32
    let sprite_bundle = SpriteBundle {
        texture: asset_server.load("character/Pink_Monster.png"),
        transform: Transform::from_xyz(20.0, 200.0, 0.0),
        ..Default::default()
    };

    commands.spawn((
        sprite_bundle,
        GamePlayer,
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
    ));
}

pub fn player_handle_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut rigid_body: Query<&mut Velocity, With<GamePlayer>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut rb in rigid_body.iter_mut() {
            rb.linvel.y = 100.0;
        }
    }
}
