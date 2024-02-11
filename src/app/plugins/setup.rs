//! This file defines the `SetupPlugin` struct, a Bevy plugin responsible for setting up common behaviors
//! such as window icon configuration, keyboard logging, and adding basic shortcuts to the application.

use bevy::{
    prelude::*,
    winit::WinitWindows,
    input::{
        keyboard::KeyCode,
        Input
    },
};
use winit::window::Icon;


/// Plugin for setting up common behaviors like window icon, keyboard logging, and shortcuts.
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, set_window_icon)
            // .add_systems(Update, keyboard_log)
            .add_systems(Update, add_shortcuts)
            .add_systems(Startup, spawn_camera)
            // This is a default impl of the DefaultPlugins but with a custom window
            // This allows us to set the window title and resize constraints
            .add_plugins(DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Iron TankTics".to_string(),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 1080.0, min_height: 720.0,
                            max_width: 1920.0, max_height: 1080.0,
                        },
                        ..default()
                    }),
                    ..default()
                }
            ))
            ;
    }
}

fn spawn_camera(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera3dBundle::default());
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// System to set the window icon for all windows.
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/tank.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    windows.windows.values().for_each(|window| window.set_window_icon(
        Some(Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height).unwrap())
    ));
}

/// System to print on the console the key that is pressed.
fn keyboard_log(input: Res<Input<KeyCode>>) {
    if let Some(key) = input.get_just_pressed().last() {
        println!("Just pressed: {:?}", key);
    }
}

/// System to add basic shortcuts to the app.
fn add_shortcuts(keyboard_input: ResMut<Input<KeyCode>>) {
    if let Some(key) = keyboard_input.get_just_pressed().last() {
        match key {
            KeyCode::F1 => println!("F1 pressed!"),
            KeyCode::Escape => std::process::exit(0),
            _ => {}
        }
    }
}
