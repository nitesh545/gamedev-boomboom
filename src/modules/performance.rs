use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::modules::player::{Player, PlayerTurret, TurretParent, TurretSide};

pub fn monitor_fps(diagnostics: Res<DiagnosticsStore>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                println!("FPS: {:.2}", fps_smoothed);
            }
        }
    }
}

pub fn debug_turret_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    turret_query: Query<(Entity, &Transform, &PlayerTurret, &TurretParent)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        println!("=== TURRET DEBUG INFO ===");

        if let Ok((player_entity, player_transform)) = player_query.single() {
            println!("Player Entity: {player_entity:?}");
            println!("Player Position: {:?}", player_transform.translation);
            println!(
                "Player Rotation: {:?}",
                player_transform.rotation.to_euler(EulerRot::ZYX).0
            );
        }

        println!("Total Turrets: {}", turret_query.iter().count());

        for (entity, transform, turret, parent) in turret_query.iter() {
            println!("Turret {entity:?}:");
            println!("  Side: {:?}", turret.side);
            println!("  Position: {:?}", transform.translation);
            println!("  Angle: {:.2}", turret.current_angle);
            println!("  Parent: {:?}", parent.0);
            println!("  ---");
        }
        println!("========================");
    }
}

pub fn visual_turret_debug(
    mut gizmos: Gizmos,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    turret_query: Query<(&Transform, &PlayerTurret)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerTurret>)>,
) {
    if keyboard_input.pressed(KeyCode::F3) {
        if let Ok(player_transform) = player_query.single() {
            let player_pos = player_transform.translation.truncate();

            for (turret_transform, turret) in turret_query.iter() {
                let turret_pos = turret_transform.translation.truncate();

                let color = match turret.side {
                    TurretSide::Left => Color::srgb(0.0, 0.0, 1.0),
                    TurretSide::Right => Color::srgb(1.0, 0.0, 0.0),
                };

                gizmos.line_2d(player_pos, turret_pos, color);

                let facing_dir = Vec2::new(turret.current_angle.cos(), turret.current_angle.sin());
                let end_pos = turret_pos + facing_dir * 30.0;
                gizmos.line_2d(turret_pos, end_pos, Color::srgb(1.0, 1.0, 0.0));
            }
        }
    }
}

// Stub implementations for other performance systems
pub fn detailed_performance_report() {}
pub fn performance_optimization_hints() {}
pub fn performance_toggles() {}
pub fn adjust_fps_target() {}
pub fn limit_entity_counts() {}
pub fn aggressive_distance_culling() {}
pub fn cycle_performance_mode() {}
pub fn conditional_system_disabling() {}
pub fn emergency_performance_recovery() {}
pub fn adaptive_quality_scaling() {}
