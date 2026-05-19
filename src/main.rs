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
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::modules::player::Player;
// use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
// use std::time::Duration;
// use vleue_kinetoscope::AnimatedImagePlugin;

mod modules;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct ExplodeTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FreeCameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (spawn_camera, spawn_light, load_ground_3d, setup_player),
        )
        .add_systems(
            Update,
            (
                move_player,
                jump_mechanic,
                dash_mechanic,
                spawn_bomb,
                bomb_explode,
            ),
        )
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 100000.0,
            ..Default::default()
        },
        // Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

pub fn load_ground_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(100.0, 5.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, -2.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(50.0, 2.5, 50.0),
    ));
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(100, 0, 100))),
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(0.25, 0.75, 0.25),
        Velocity::linear(Vec3::splat(0.0)),
        Restitution {
            coefficient: 1.0,
            ..Default::default()
        },
    ));
}

// GDB-1-player-movement
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction_x = 0.0;
    let direction_y = 0.0;
    let mut direction_z = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_z += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_z -= 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position_x: f32 =
        player_transform.translation.x + direction_x * 25.0 * time.delta_secs();
    let new_player_position_y: f32 =
        player_transform.translation.y + direction_y * 25.0 * time.delta_secs();
    let new_player_position_z: f32 =
        player_transform.translation.z + direction_z * 25.0 * time.delta_secs();

    player_transform.translation = Vec3::new(
        new_player_position_x,
        new_player_position_y,
        new_player_position_z,
    );
}

// GDB-2-jump-mechanic
// Jump mechanic - Player
pub fn jump_mechanic(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Query<&mut Velocity>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for mut vel in player_velocity.iter_mut() {
            vel.linear = Vec3::new(0.0, 20.0, 0.0);
        }
    }
}

// GDB-3-ability-dash
// Dash mechanic - Player
// TODO: Ability is present but not complete
// TODO: need to fix the direction to dash to
pub fn dash_mechanic(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Query<&mut Velocity>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let default_transform = &Transform::default();
    let transform = player_transform.single().unwrap_or(default_transform);
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut vel in player_velocity.iter_mut() {
            let forward = transform.forward();
            vel.linear = Vec3::new(forward.x * -100.0, -forward.y, forward.z * -100.0);
        }
    }
}

// GDB-3-bomb-placement-and-explosion
// Bomb placement & Explosion - Player

// Part-1: Bomb Placement
pub fn spawn_bomb(
    mut command: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_transform: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let default_transform = &Transform::default();
    let transform = player_transform.single().unwrap_or(default_transform);
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        command.spawn((
            Transform::from(transform.to_owned()),
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(Color::srgb_u8(0, 100, 0))),
            Bomb,
            RigidBody::KinematicPositionBased,
            Collider::ball(2.0),
            Sensor,
            ExplodeTimer(Timer::from_seconds(2.0, TimerMode::Once)),
        ));
    }
}

// Part-2: Explosion
pub fn bomb_explode(
    mut command: Commands,
    time: Res<Time>,
    mut bomb: Query<(Entity, &mut ExplodeTimer), With<Bomb>>,
) {
    for (entity, mut timer) in bomb.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            command.entity(entity).despawn();
        }
    }
}
