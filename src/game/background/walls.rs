use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct GameWall;

pub fn setup_walls(mut commands: Commands) {
    let width = 50.0;
    let height = 1500.0;

    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-(720.0 / 2.0), height / 2.0, 0.0)),
            ..default()
        },
        GameWall,
        Collider::cuboid(width / 2.0, height / 2.0),
        RigidBody::Fixed,
    ));

    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(720.0 / 2.0, height / 2.0, 0.0)),
            ..default()
        },
        GameWall,
        Collider::cuboid(width / 2.0, height / 2.0),
        RigidBody::Fixed,
    ));
}
