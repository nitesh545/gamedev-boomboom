#![allow(dead_code)]
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub start_color: Color,
    pub end_color: Color,
    pub start_size: f32,
    pub end_size: f32,
    pub fade_over_time: bool,
}

impl Particle {
    pub fn new(
        lifetime_secs: f32,
        start_color: Color,
        end_color: Color,
        start_size: f32,
        end_size: f32,
    ) -> Self {
        Self {
            lifetime: Timer::new(Duration::from_secs_f32(lifetime_secs), TimerMode::Once),
            start_color,
            end_color,
            start_size,
            end_size,
            fade_over_time: true,
        }
    }

    pub fn explosion_spark() -> Self {
        Self::new(
            0.8,
            Color::srgb(1.0, 1.0, 0.8),
            Color::srgb(1.0, 0.3, 0.0),
            4.0,
            1.0,
        )
    }

    pub fn explosion_smoke() -> Self {
        Self::new(
            1.5,
            Color::srgba(0.4, 0.3, 0.2, 0.8),
            Color::srgba(0.2, 0.2, 0.2, 0.0),
            8.0,
            20.0,
        )
    }

    pub fn engine_flame() -> Self {
        Self::new(
            0.3,
            Color::srgb(0.2, 0.6, 1.0),
            Color::srgba(0.8, 0.9, 1.0, 0.0),
            6.0,
            2.0,
        )
    }

    pub fn debris_spark() -> Self {
        Self::new(
            0.5,
            Color::srgb(1.0, 0.8, 0.4),
            Color::srgba(0.6, 0.2, 0.1, 0.0),
            2.0,
            0.5,
        )
    }
}

#[derive(Component)]
pub struct ParticleBehavior {
    pub gravity: Vec2,
    pub drag: f32,
    pub spin_speed: f32,
}

impl ParticleBehavior {
    pub fn explosion() -> Self {
        Self {
            gravity: Vec2::new(0.0, -50.0),
            drag: 0.98,
            spin_speed: (fastrand::f32() - 0.5) * 10.0,
        }
    }

    pub fn engine_exhaust() -> Self {
        Self {
            gravity: Vec2::ZERO,
            drag: 0.95,
            spin_speed: 0.0,
        }
    }

    pub fn floating_debris() -> Self {
        Self {
            gravity: Vec2::new(0.0, -20.0),
            drag: 0.99,
            spin_speed: (fastrand::f32() - 0.5) * 5.0,
        }
    }
}

#[derive(Component)]
pub struct ExplosionParticle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub initial_size: f32,
}

use crate::modules::enemies::Enemy;
use crate::modules::player::{Player, PlayerStats, PlayerTurret};

//pub fn spawn_explosion_particles_optimized(
//    mut commands: Commands,
//    mut collision_events: EventReader<CollisionEvent>,
//    bullet_query: Query<&Transform, (With<Bullet>, Without<Meteor>, Without<Enemy>)>,
//    meteor_query: Query<&Transform, (With<Meteor>, Without<Bullet>, Without<Enemy>)>,
//    enemy_query: Query<&Transform, (With<Enemy>, Without<Bullet>, Without<Meteor>)>,
//) {
//    for collision_event in collision_events.read() {
//        if let CollisionEvent::Started(h1, h2, _) = collision_event {
//            let entities = [*h1, *h2];
//            let mut explosion_pos = None;
//            let mut explosion_size = 1.0;
//
//            for entity in entities {
//                if bullet_query.contains(entity) {
//                    if let Ok(bullet_transform) = bullet_query.get(entity) {
//                        explosion_pos = Some(bullet_transform.translation.truncate());
//                    }
//                }
//                if meteor_query.contains(entity) {
//                    if let Ok(meteor_transform) = meteor_query.get(entity) {
//                        explosion_pos = Some(meteor_transform.translation.truncate());
//                        explosion_size = 2.0;
//                    }
//                }
//                if enemy_query.contains(entity) {
//                    if let Ok(enemy_transform) = enemy_query.get(entity) {
//                        explosion_pos = Some(enemy_transform.translation.truncate());
//                        explosion_size = 1.5;
//                    }
//                }
//            }
//
//            if let Some(pos) = explosion_pos {
//                spawn_explosion_at(&mut commands, pos, explosion_size);
//            }
//        }
//    }
//}

fn spawn_explosion_at(commands: &mut Commands, position: Vec2, size_multiplier: f32) {
    let particle_count = (10.0 * size_multiplier) as usize;

    // Spawn spark particles
    for _ in 0..particle_count {
        // let angle: f32 = fastrand::f32() * 2.0 * std::f32::consts::PI;
        // let speed: f32 = 100.0 + fastrand::f32() * 200.0 * size_multiplier;
        // let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 0.8),
                custom_size: Some(Vec2::splat(4.0 * size_multiplier)),
                ..default()
            },
            Transform::from_translation(position.extend(990.0)),
            Particle::explosion_spark(),
            ParticleBehavior::explosion(),
            // NOTE: use velocity when we have bevy_rapier2d crate imported
            // Velocity(Velocity),
        ));
    }

    // Spawn smoke particles
    for _ in 0..(particle_count / 2) {
        // let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
        // let speed = 30.0 + fastrand::f32() * 60.0;
        // let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        commands.spawn((
            Sprite {
                color: Color::srgba(0.4, 0.3, 0.2, 0.8),
                custom_size: Some(Vec2::splat(8.0 * size_multiplier)),
                ..default()
            },
            Transform::from_translation(position.extend(990.0)),
            Particle::explosion_smoke(),
            ParticleBehavior::explosion(),
            // NOTE: use velocity when we have bevy_rapier2d crate imported
            // Velocity(Velocity),
        ));
    }
}

pub fn spawn_engine_particles_physics_based(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let is_moving = keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyS)
        || keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyD)
        || keyboard_input.pressed(KeyCode::ArrowUp)
        || keyboard_input.pressed(KeyCode::ArrowDown)
        || keyboard_input.pressed(KeyCode::ArrowLeft)
        || keyboard_input.pressed(KeyCode::ArrowRight);

    if !is_moving || fastrand::f32() > 0.3 {
        return;
    }

    if let Ok(player_transform) = player_query.single() {
        let player_rotation = player_transform.rotation.to_euler(EulerRot::ZYX).0;
        let rear_local = Vec2::new(0.0, -1.0);

        let cos_rot = player_rotation.cos();
        let sin_rot = player_rotation.sin();
        let rear_world = Vec2::new(
            rear_local.x * cos_rot - rear_local.y * sin_rot,
            rear_local.x * sin_rot + rear_local.y * cos_rot,
        );

        let ship_center = player_transform.translation.truncate();
        let rear_center = ship_center + rear_world * 15.0;
        let side_direction = Vec2::new(-rear_world.y, rear_world.x);

        let engine_positions = [
            rear_center + side_direction * 6.0,
            rear_center - side_direction * 6.0,
        ];

        for engine_pos in engine_positions {
            // let exhaust_direction = rear_world
            //     + Vec2::new((fastrand::f32() - 0.5) * 0.4, (fastrand::f32() - 0.5) * 0.4);
            // let particle_speed = 120.0 + fastrand::f32() * 80.0;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0),
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
                Transform::from_translation(engine_pos.extend(980.0)),
                Particle::engine_flame(),
                ParticleBehavior::engine_exhaust(),
                // Velocity(exhaust_direction * particle_speed),
            ));
        }
    }
}

pub fn spawn_muzzle_flash_particles(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    turret_query: Query<(&Transform, &PlayerTurret)>,
) {
    let should_shoot =
        mouse_input.just_pressed(MouseButton::Left) || keyboard_input.just_pressed(KeyCode::Space);

    if !should_shoot {
        return;
    }

    for (turret_transform, turret) in turret_query.iter() {
        let turret_direction = Vec2::new(turret.current_angle.cos(), turret.current_angle.sin());
        let muzzle_pos = turret_transform.translation.truncate() + turret_direction * 12.0;

        for _ in 0..10 {
            let flash_offset =
                Vec2::new((fastrand::f32() - 0.5) * 8.0, (fastrand::f32() - 0.5) * 8.0);

            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 1.0, 0.8),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                Transform::from_translation((muzzle_pos + flash_offset).extend(990.0)),
                Particle::new(
                    0.1,
                    Color::srgb(1.0, 1.0, 1.0),
                    Color::srgba(1.0, 0.5, 0.0, 0.0),
                    10.0,
                    2.0,
                ),
                ParticleBehavior {
                    gravity: Vec2::ZERO,
                    drag: 0.8,
                    spin_speed: (fastrand::f32() - 0.5) * 20.0,
                },
                // Velocity(turret_direction * (50.0 + fastrand::f32() * 100.0)),
            ));
        }
    }
}

//pub fn spawn_impact_particles(
//    mut commands: Commands,
//    mut collision_events: EventReader<CollisionEvent>,
//    bullet_query: Query<&Transform, With<Bullet>>,
//    meteor_query: Query<(), With<Meteor>>,
//    enemy_query: Query<(), With<Enemy>>,
//) {
//    for collision_event in collision_events.read() {
//        if let CollisionEvent::Started(h1, h2, _) = collision_event {
//            let entities = [*h1, *h2];
//            let mut bullet_pos = None;
//            let mut hit_something = false;
//
//            for entity in entities {
//                if let Ok(bullet_transform) = bullet_query.get(entity) {
//                    bullet_pos = Some(bullet_transform.translation.truncate());
//                }
//                if meteor_query.contains(entity) || enemy_query.contains(entity) {
//                    hit_something = true;
//                }
//            }
//
//            if let (Some(pos), true) = (bullet_pos, hit_something) {
//                for _ in 0..6 {
//                    let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
//                    let speed = 80.0 + fastrand::f32() * 120.0;
//                    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
//
//                    commands.spawn((
//                        Sprite {
//                            color: Color::srgb(1.0, 0.8, 0.4),
//                            custom_size: Some(Vec2::splat(3.0)),
//                            ..default()
//                        },
//                        Transform::from_translation(pos.extend(990.0)),
//                        Particle::debris_spark(),
//                        ParticleBehavior::explosion(),
//                        // NOTE: use velocity when we have bevy_rapier2d crate imported
//                        // Velocity(Velocity),
//                    ));
//                }
//            }
//        }
//    }
//}

pub fn spawn_damage_particles(mut commands: Commands, enemy_query: Query<(&Transform, &Enemy)>) {
    for (transform, enemy) in enemy_query.iter() {
        if enemy.health < 2 && fastrand::f32() < 0.1 {
            let pos = transform.translation.truncate();
            let offset = Vec2::new(
                (fastrand::f32() - 0.5) * 20.0,
                (fastrand::f32() - 0.5) * 20.0,
            );

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.8, 0.4, 0.2),
                    custom_size: Some(Vec2::splat(2.0)),
                    ..default()
                },
                Transform::from_translation((pos + offset).extend(988.0)),
                Particle::new(
                    1.0,
                    Color::srgb(0.8, 0.4, 0.2),
                    Color::srgba(0.2, 0.1, 0.1, 0.0),
                    3.0,
                    1.0,
                ),
                ParticleBehavior::floating_debris(),
                //Velocity(Vec2::new(
                //    (fastrand::f32() - 0.5) * 40.0,
                //    (fastrand::f32() - 0.5) * 40.0,
                //)),
            ));
        }
    }
}

pub fn spawn_shield_particles(
    mut commands: Commands,
    player_query: Query<(&Transform, &PlayerStats), With<Player>>,
) {
    if let Ok((player_transform, stats)) = player_query.single() {
        if stats.shield_active && fastrand::f32() < 0.4 {
            let player_pos = player_transform.translation.truncate();
            let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
            let radius = 25.0 + fastrand::f32() * 10.0;
            let particle_pos = player_pos + Vec2::new(angle.cos(), angle.sin()) * radius;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.8, 1.0),
                    custom_size: Some(Vec2::splat(4.0)),
                    ..default()
                },
                Transform::from_translation(particle_pos.extend(983.0)),
                Particle::new(
                    0.8,
                    Color::srgba(0.3, 0.8, 1.0, 0.8),
                    Color::srgba(0.1, 0.4, 0.6, 0.0),
                    5.0,
                    1.0,
                ),
                ParticleBehavior {
                    gravity: Vec2::ZERO,
                    drag: 0.96,
                    spin_speed: 5.0,
                },
                //Velocity(Vec2::new(
                //    (fastrand::f32() - 0.5) * 30.0,
                //    (fastrand::f32() - 0.5) * 30.0,
                //)),
            ));
        }
    }
}

pub fn spawn_ambient_particles(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    if fastrand::f32() < 0.02 {
        if let Ok(player_transform) = player_query.single() {
            let player_pos = player_transform.translation.truncate();
            let offset = Vec2::new(
                (fastrand::f32() - 0.5) * 1200.0,
                (fastrand::f32() - 0.5) * 800.0,
            );
            let particle_pos = player_pos + offset;

            commands.spawn((
                Sprite {
                    color: Color::srgba(0.8, 0.8, 1.0, 0.3),
                    custom_size: Some(Vec2::splat(1.0 + fastrand::f32() * 2.0)),
                    ..default()
                },
                Transform::from_translation(particle_pos.extend(900.0)),
                Particle::new(
                    10.0 + fastrand::f32() * 20.0,
                    Color::srgba(0.8, 0.8, 1.0, 0.3),
                    Color::srgba(0.4, 0.4, 0.6, 0.0),
                    2.0,
                    0.5,
                ),
                ParticleBehavior {
                    gravity: Vec2::ZERO,
                    drag: 0.999,
                    spin_speed: (fastrand::f32() - 0.5) * 1.0,
                },
                //Velocity(Vec2::new(
                //    (fastrand::f32() - 0.5) * 20.0,
                //    (fastrand::f32() - 0.5) * 20.0,
                //)),
            ));
        }
    }
}

pub fn update_particles(
    mut commands: Commands,
    mut particle_query: Query<(
        Entity,
        &mut Sprite,
        &mut Transform,
        &mut Particle,
        &ParticleBehavior,
    )>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut transform, mut particle, behavior) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let t = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();

        if particle.fade_over_time {
            sprite.color = Color::srgba(
                particle.start_color.to_srgba().red * (1.0 - t)
                    + particle.end_color.to_srgba().red * t,
                particle.start_color.to_srgba().green * (1.0 - t)
                    + particle.end_color.to_srgba().green * t,
                particle.start_color.to_srgba().blue * (1.0 - t)
                    + particle.end_color.to_srgba().blue * t,
                particle.start_color.to_srgba().alpha * (1.0 - t)
                    + particle.end_color.to_srgba().alpha * t,
            );
        }

        let current_size = particle.start_size * (1.0 - t) + particle.end_size * t;
        sprite.custom_size = Some(Vec2::splat(current_size));

        transform.rotation *= Quat::from_rotation_z(behavior.spin_speed * time.delta_secs());
    }
}

pub fn cleanup_distant_particles(
    mut commands: Commands,
    particle_query: Query<(Entity, &Transform), With<Particle>>,
    player_query: Query<&Transform, (With<Player>, Without<Particle>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_pos = player_transform.translation.truncate();

        for (entity, transform) in particle_query.iter() {
            let particle_pos = transform.translation.truncate();
            let distance = (particle_pos - player_pos).length();

            if distance > 10.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn create_impact_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    intensity: f32,
) {
    // Create multiple explosion particles
    for i in 0..((intensity * 10.0) as usize).max(5).min(20) {
        let angle = (i as f32 / (intensity * 10.0)) * std::f32::consts::TAU;
        let speed = fastrand::f32() * 200.0 + 100.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

        let size = fastrand::f32() * 4.0 + 2.0;
        let color = Color::srgb(
            1.0,                         // Full red
            0.3 + fastrand::f32() * 0.7, // 0.3-1.0 green
            fastrand::f32() * 0.3,       // 0-0.3 blue
        );

        commands.spawn((
            Mesh2d(meshes.add(Mesh::from(Circle::new(size))).into()),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_translation(position.extend(992.0)),
            ExplosionParticle {
                velocity,
                lifetime: Timer::from_seconds(fastrand::f32() * 0.5 + 0.3, TimerMode::Once),
                initial_size: size,
            },
        ));
    }
}

pub fn update_explosion_particles(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Transform, &mut ExplosionParticle)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Move particle
        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();

        // Shrink over time
        let progress = particle.lifetime.fraction();
        let current_size = particle.initial_size * (1.0 - progress);
        transform.scale = Vec3::splat(current_size / particle.initial_size);

        // Slow down particle
        particle.velocity *= 0.98; // 2% drag per frame
    }
}
