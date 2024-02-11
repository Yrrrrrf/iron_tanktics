pub mod map;
use map::*;

pub mod plugins;
// use plugins::*;

extern crate bevy;
use bevy::prelude::*;

pub fn init_bevy_app() {
    App::new()
        .add_plugins(plugins::setup::SetupPlugin)
        // .add_systems(Startup, spawn_tank)
        // .add_systems(Update, (update_position, print_position))


        // .add_systems(Update, (system, update_config))
        .insert_resource(ClearColor(Color::rgb_u8(32, 32, 32)))
        .add_systems(Startup, spawn_basic_scene)

        .run();
}


fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Draw a floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, subdivisions: 5})),
        material: materials.add(Color::rgb_u8(32, 128, 64).into()),
        ..default()
    });

    // Draw a cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::rgb(3.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    // Spawn a red cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        // material: materials.add(Color::rgb_u8(0, 0, 255).into()),
        material: materials.add(Color::rgb(0.0, 0.2, 0.2).into()),
        transform: Transform::from_xyz(2.0, 1.0, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..default()
    });

}


// #[derive(Debug, Component)]
// struct Position {
//     x: f32,
//     y: f32,
// }

// fn spawn_tank(mut commands: Commands) {
//     commands.spawn(Position { x: 0.0, y: 0.0 });
// }

// fn update_position(mut query: Query<(&mut Position)>) {
//     for (mut position) in query.iter_mut() {
//         position.x += 1.0;
//         position.y += 1.0;
//     }
// }

// fn print_position(query: Query<(&Position)>) {
//     for (position) in query.iter() {
//         info!("Position: x: {}, y: {}", position.x, position.y);
//     }
// }

// #[derive(Debug, Resource, Default)]
// pub enum GameState {
//     #[default]
//     Running,
//     Paused,
//     GameOver,  // Can be a victory or a defeat
// }

// pub fn set_app_state(mut game_state: ResMut<GameState>, input: Res<Input<KeyCode>>) {
//     if input.just_pressed(KeyCode::Space) {
//         match *game_state {
//             GameState::Running => *game_state = GameState::Paused,
//             GameState::Paused => *game_state = GameState::Running,
//             GameState::GameOver => *game_state = GameState::Running,
//         }
//     }
// }
