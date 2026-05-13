use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MouseWorldPos(pub Vec2);

use bevy::app::AppExit;
use bevy::window::{PrimaryWindow, WindowMode};

pub fn escape_key_handler(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.trigger(AppExit::Success);
    }
}

pub fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => {
                    println!("Switching to fullscreen");
                    WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                }
                _ => {
                    println!("Switching to windowed");
                    WindowMode::Windowed
                }
            };
        }
    }
}
