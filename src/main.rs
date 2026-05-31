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
use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;

use crate::modules::enemies::{Enemy, EnemyBehavior, EnemyType};
use crate::modules::player::Player;
use crate::modules::timers::EnemySpawnTimer;
use crate::modules::{Health, HealthBar, HealthBarFill};
// use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
// use std::time::Duration;
use vleue_kinetoscope::{AnimatedImageController, AnimatedImagePlugin};

mod modules;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct SkyBirdFlock;

#[derive(Component)]
pub struct ExplodeTimer(Timer);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::linear(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(AnimatedImagePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FreeCameraPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(EnemySpawnTimer::default())
        .add_systems(
            Startup,
            (
                spawn_2dcamera,
                play_bgmusic,
                setup_player,
                spawn_background,
                animated_details,
                // spawn_slope_collider,
            ),
        )
        //.add_systems(
        //    Update,
        //    (spawn_enemies, give_enemy_a_body, enemy_ai_behavior),
        //)
        .add_systems(
            Update,
            (
                clamp_player,
                move_player,
                birds_fly,
                // jump_mechanic,
                // dash_mechanic,
                // spawn_bomb,
                // bomb_explode,
            ),
        )
        .run();
}

const X_MIN: f32 = -920.0;
const X_MAX: f32 = 920.0;

pub fn get_y_bounds(x: f32) -> (f32, f32) {
    // y bounds shift depending on where on screen player is
    // tune these numbers by eyeballing over your image
    let t = (x - X_MIN) / (X_MAX - X_MIN); // 0.0 to 1.0 across screen

    let y_min = f32::lerp(-600.0, -700.0, t); // floor of walkable zone
    let y_max = f32::lerp(-500.0, -625.0, t); // ceiling (hill ridge)

    (y_min, y_max)
}

pub fn clamp_player(mut query: Query<&mut Transform, With<Player>>) {
    // Transform::from_xyz(-900., -525., 0.1).with_scale(Vec3::splat(0.15)),
    for mut transform in &mut query {
        let x = transform.translation.x.clamp(X_MIN, X_MAX);
        let (y_min, y_max) = get_y_bounds(x);
        let y = transform.translation.y.clamp(y_min, y_max);

        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn animated_details(mut commands: Commands, asset_server: Res<AssetServer>) {
    let deer_positions = vec![
        (
            Vec3::new(289., -503., 0.1),
            0.1,
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        ),
        (
            Vec3::new(-304.5, -499.5, 0.1),
            0.1,
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
        ),
        (
            Vec3::new(1221.5, -401.1, 0.1),
            0.1,
            Quat::from_euler(EulerRot::XYZ, 0.0, 3.0, 0.0),
        ),
    ];
    for transform in deer_positions {
        commands.spawn((
            AnimatedImageController::play(asset_server.load("shadedeer.gif")),
            Sprite {
                color: Color::srgb_u8(0x1f, 0x3f, 0x66),
                ..Default::default()
            },
            Transform::from_xyz(transform.0.x, transform.0.y, transform.0.z)
                .with_scale(Vec3::splat(transform.1))
                .with_rotation(transform.2),
        ));
    }

    commands.spawn((
        Sprite {
            image: asset_server.load("birds.png"),
            color: Color::srgb_u8(0xdd, 0xd2, 0xb5),
            ..Default::default()
        },
        Transform::from_xyz(-1300., 400., 0.1).with_scale(Vec3::new(0.55, 0.3, 1.0)),
        SkyBirdFlock,
    ));
}

pub fn birds_fly(birds: Query<&mut Transform, With<SkyBirdFlock>>, time: Res<Time>) {
    for mut bird_flock_transform in birds {
        bird_flock_transform.translation.x += time.delta_secs() * 10.0;
    }
}

pub fn play_bgmusic(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/bgmusic1.ogg")),
        PlaybackSettings::LOOP
            .with_volume(bevy::audio::Volume::Linear(0.5))
            .with_start_position(Duration::from_secs(1)),
    ));
}

pub fn spawn_2dcamera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        // Transform::from_xyz(8.382, 18.700, 26.902).with_rotation(Quat::from_euler(
        //     EulerRot::XYZ,
        //     0.093,
        //     -0.054,
        //     -0.005,
        // )),
        Transform::from_xyz(0., 0., 0.),
        FreeCamera::default(),
    ));
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("bg1.png"),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.67)),
    ));
}

fn spawn_slope_collider(mut commands: Commands) {
    // X = horizontal, Y = height of slope at that X
    // Tune these values to match your image visually
    let points = vec![
        Vec2::new(-6.1, 19.5),
        Vec2::new(-5.0, 19.5),
        Vec2::new(-4.0, 19.5),
        Vec2::new(0.0, 19.5),
        Vec2::new(5.0, 19.5),
        Vec2::new(4.0, 19.5),
        Vec2::new(10.0, 19.5),
    ];

    // Build indices — each consecutive pair is a segment
    let indices: Vec<[u32; 2]> = (0..points.len() as u32 - 1).map(|i| [i, i + 1]).collect();

    commands.spawn((
        Collider::polyline(points, Some(indices)),
        RigidBody::Fixed,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            ..Default::default()
        },
        Transform::from_xyz(-900., -525., 0.1).with_scale(Vec3::splat(0.15)),
        Player,
    ));
}

// GDB-1-player-movement
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let speed: f32 = 100.0;
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
        player_transform.translation.x + direction_x * speed * time.delta_secs();
    let new_player_position_y: f32 =
        player_transform.translation.y + direction_y * speed * time.delta_secs();

    player_transform.translation = Vec3::new(new_player_position_x, new_player_position_y, 0.1);
}

// GDB-2-jump-mechanic
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
            vel.linear = Vec2::new(forward.x * -100.0, -forward.y * -100.0);
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
        Velocity::linear(Vec2::splat(0.0)),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
        Health::new(100.0),
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
    let player_pos = player_transform.translation.truncate();

    for (mut velocity, mut behavior, enemy, transform) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
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
                    velocity.linear = Vec2::ZERO;
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
                    let perpendicular = Vec2::new(-to_player.y, to_player.x).normalize();
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
