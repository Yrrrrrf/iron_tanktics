pub mod map;
use map::*;

extern crate bevy;
use bevy::prelude::*;

use bevy::ecs::system::{NonSend, Res};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;
use bevy::winit::WinitWindows;

use winit::window::Icon;


pub fn init_bevy_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, set_window_icon)
        .add_systems(Update, add_shortcuts)
        .add_systems(Update, keyboard_log)

        .add_systems(Startup, spawn_tank)
        .add_systems(Update, (update_position, print_position))


        // .add_systems(Startup, setup)
        // .add_systems(Update, (system, update_config))
        .run();
}




#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

fn spawn_tank(mut commands: Commands) {
    commands.spawn(Position { x: 0.0, y: 0.0 });
}

fn update_position(mut query: Query<(&mut Position)>) {
    for (mut position) in query.iter_mut() {
        position.x += 1.0;
        position.y += 1.0;
    }
}

fn print_position(query: Query<(&Position)>) {
    for (position) in query.iter() {
        info!("Position: x: {}, y: {}", position.x, position.y);
    }
}




#[derive(Debug, Resource, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,  // Can be a victory or a defeat
}
















/// Sets the window icon for all windows
/// 
/// # Parameters
/// - `windows`: NonSend<WinitWindows>
/// 
/// # Returns
/// Nothing
/// 
/// # Panics
/// Panics if the icon path is invalid
pub fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/tank_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    windows.windows.values().for_each(|window| 
        window.set_window_icon(Some(Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height).unwrap()))
    );
}


/// Print on console the key that is pressed
/// 
/// # Parameters
/// - `input`: Res<Input<KeyCode>>
/// 
/// # Returns
/// Nothing
/// 
/// # Panics
/// Nothing
pub fn keyboard_log(input: Res<Input<KeyCode>>) {
    match input.get_just_pressed().last() {
        Some(key) => println!("Just pressed: {:?}", key),
        None => {}
    }
}


/// Add some basic shortcuts to the app
/// 
pub fn add_shortcuts(mut keyboard_input: ResMut<Input<KeyCode>>) {
    match keyboard_input.get_just_pressed().last() {
        Some(key) => match key {
            KeyCode::Escape => std::process::exit(0),
            _ => {}
        },
        None => {}
    }
}
