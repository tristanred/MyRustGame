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

/*
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, game::camera::static_camera::setup)
        .add_systems(Startup, game::background::floor::setup)
        .add_systems(Update, game::camera::static_camera::keyboard_input_system)
        .add_systems(PostStartup, game::camera::static_camera::print_camera_dims)
        .on_state_enter(AppState::Stage, Startup, setup_system.system())
        .run();
}

fn setup_system(mut app_state: ResMut<State<AppState>>, windows: Res<Windows>, query: Query<&Camera>) {
    // Get the first camera in the query
    if let Some(camera) = query.iter().next() {
        // Get the dimensions of the window
        if let Some(window) = windows.get(camera.window) {
            let dimensions = window.physical_size();
            println!("Window dimensions: {} x {}", dimensions.width, dimensions.height);
        }
    }
    // Move to the next state
    app_state.set(AppState::Game).unwrap();
}
*/

pub fn print_camera_dims(q: Query<&Camera>) {
    let camera = q.single();

    // the size of the area being rendered to
    let view_dimensions = camera.logical_viewport_size().unwrap();
    debug!("VP Size {:?}", view_dimensions);

    // the top-left and bottom-right coordinates
    let rec = camera.logical_viewport_rect().unwrap();

    debug!("VP Rect {:?}", rec);
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
