#![allow(clippy::all)]
// More pedantic/stylistic lints
#![allow(clippy::pedantic)]
// Experimental lints (may have false positives)
#![allow(clippy::nursery)]
// Cargo-related lints
#![allow(clippy::cargo)]
// Performance-related lints
#![allow(clippy::perf)]
// Complexity lints
#![allow(clippy::complexity)]
// Style lints
#![allow(clippy::style)]
// Correctness lints (usually errors)
#![allow(clippy::correctness)]
// Suspicious code patterns
#![allow(clippy::suspicious)]

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::modules::player::Player;
// use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
// use std::time::Duration;
// use vleue_kinetoscope::AnimatedImagePlugin;

mod modules;

#[derive(Component)]
pub struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (load_ground, setup_player))
        .add_systems(Update, (move_player, jump_mechanic))
        .run();
}

pub fn load_ground(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
        Sprite {
            image: asset_server.load("ground_proto1.png"),
            ..Default::default()
        },
    ));
}

pub fn setup_player(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(0.5)),
        Sprite {
            image: asset_server.load("player_test.png"),
            ..Default::default()
        },
        Player,
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(60.0, 85.0),
        Velocity::linear(Vec2::splat(0.0)),
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let (width, height) = match windows.single() {
        Ok(val) => (val.width(), val.height()),
        Err(_error) => (1280.0, 720.0),
    };
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_y -= 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position_x: f32 =
        player_transform.translation.x + direction_x * 100.0 * time.delta_secs();
    let new_player_position_y: f32 =
        player_transform.translation.y + direction_y * 100.0 * time.delta_secs();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = -(width / 2.0);
    let right_bound = width / 2.0;
    let down_bound = -(height / 2.0);
    let up_bound = height / 2.0;

    player_transform.translation = Vec3::new(
        new_player_position_x.clamp(left_bound, right_bound),
        new_player_position_y.clamp(down_bound, up_bound),
        1.0,
    );
}

// Jump mechanic - Player
pub fn jump_mechanic(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Query<&mut Velocity>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for mut vel in player_velocity.iter_mut() {
            vel.linear = Vec2::new(0.0, 20.0);
        }
    }
}
