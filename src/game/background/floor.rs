use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct GameFloor;

pub fn setup(mut commands: Commands, cam: Query<&Camera>, _: Res<AssetServer>) {
    let width = 500.0;
    let height = 50.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
            ..default()
        },
        GameFloor,
        Collider::cuboid(width / 2.0, height / 2.0),
        RigidBody::Fixed,

    ));
}
