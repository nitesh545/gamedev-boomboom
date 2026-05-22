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

use std::time::Duration;

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::modules::enemies::{Enemy, EnemyBehavior, EnemyType};
use crate::modules::player::Player;
use crate::modules::timers::EnemySpawnTimer;
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
        .insert_resource(EnemySpawnTimer::default())
        .add_systems(
            Startup,
            (spawn_camera, spawn_light, load_ground_3d, setup_player),
        )
        .add_systems(
            Update,
            (spawn_enemies, give_enemy_a_body, enemy_ai_behavior),
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
        Mesh3d(meshes.add(Cuboid::new(10000.0, 5.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, -2.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(500.0, 2.5, 500.0),
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
        direction_z -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_z += 1.0;
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

// GDB-5-basic-enemy-ai
// basic enemy ai - move around, follow player on detection, die when the time comes
pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.just_finished() {
        if let Ok(player_transform) = player_query.single() {
            let player_pos = player_transform.translation;
            spawn_timer.wave_intensity += 0.1;
            let num_enemies = (spawn_timer.wave_intensity as usize).min(4);

            for _ in 0..num_enemies {
                spawn_single_enemy(&mut commands, player_pos, spawn_timer.wave_intensity);
            }
        }
    }
}

fn spawn_single_enemy(commands: &mut Commands, player_pos: Vec3, intensity: f32) {
    let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
    let distance = 50.0 + fastrand::f32() * 1.0;
    let spawn_pos: Vec3 =
        player_pos + Vec3::new(angle.cos() * distance, 10.0, angle.sin() * distance);

    let enemy_types = [EnemyType::Chaser, EnemyType::Flanker, EnemyType::Dasher];
    let enemy_type = enemy_types[fastrand::usize(0..enemy_types.len())];

    let (size, speed, health) = match enemy_type {
        EnemyType::Chaser => (20.0, 120.0, 1),
        EnemyType::Flanker => (16.0, 80.0, 2),
        EnemyType::Dasher => (14.0, 60.0, 1),
    };

    commands.spawn((
        Transform::from_translation(spawn_pos),
        Enemy {
            enemy_type,
            speed: speed * (1.0 + intensity * 0.1),
            health,
        },
        EnemyBehavior::default(),
        RigidBody::Dynamic,
        Collider::ball(size / 10.0),
        Velocity::linear(Vec3::splat(0.0)),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn give_enemy_a_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut enemies: Query<Entity, With<Enemy>>,
) {
    let enemy_types = [EnemyType::Chaser, EnemyType::Flanker, EnemyType::Dasher];
    let enemy_type = enemy_types[fastrand::usize(0..enemy_types.len())];
    let color = match enemy_type {
        EnemyType::Chaser => Color::srgb(1.0, 0.2, 0.2),
        EnemyType::Flanker => Color::srgb(0.8, 0.2, 0.8),
        EnemyType::Dasher => Color::srgb(0.2, 0.8, 1.0),
    };
    for enemy in enemies.iter_mut() {
        let mut enemy_entity = commands.entity(enemy);
        enemy_entity.insert((
            Mesh3d(meshes.add(Sphere::new(2.0))),
            MeshMaterial3d(materials.add(color)),
        ));
    }
}

pub fn enemy_ai_behavior(
    mut enemy_query: Query<(&mut Velocity, &mut EnemyBehavior, &Enemy, &Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    for (mut velocity, mut behavior, enemy, transform) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation;
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();

        behavior.behavior_timer.tick(time.delta());
        behavior.dash_cooldown.tick(time.delta());

        match enemy.enemy_type {
            EnemyType::Chaser => {
                if distance > 5.0 {
                    let direction = to_player.normalize();
                    velocity.linear = direction * enemy.speed;
                } else {
                    velocity.linear = Vec3::ZERO;
                }
            }
            EnemyType::Flanker => {
                if behavior.behavior_timer.just_finished() {
                    behavior.target_angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                }

                if distance > 80.0 {
                    let direction = to_player.normalize();
                    velocity.linear = direction * enemy.speed;
                } else if distance < 40.0 {
                    let direction = -to_player.normalize();
                    velocity.linear = direction * enemy.speed;
                } else {
                    let perpendicular = Vec3::new(-to_player.y, 0.0, to_player.x).normalize();
                    let circle_direction = perpendicular * behavior.target_angle.cos()
                        + to_player.normalize() * behavior.target_angle.sin() * 0.3;
                    velocity.linear = circle_direction * enemy.speed * 0.8;
                }
            }
            EnemyType::Dasher => {
                if distance > 150.0 {
                    let direction = to_player.normalize();
                    velocity.linear = direction * enemy.speed * 0.5;
                } else if distance > 50.0 && behavior.dash_cooldown.just_finished() {
                    let direction = to_player.normalize();
                    velocity.linear = direction * enemy.speed * 3.0;
                    behavior.dash_cooldown = Timer::new(Duration::from_secs(3), TimerMode::Once);
                } else {
                    velocity.linear *= 0.8;
                }
            }
        }
    }
}
