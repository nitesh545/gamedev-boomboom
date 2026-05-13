use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStats {
    pub fire_rate_multiplier: f32,
    pub movement_speed_multiplier: f32,
    pub multi_shot: bool,
    pub shield_active: bool,
    pub shield_timer: Timer,
    pub rapid_fire_timer: Timer,
    pub speed_boost_timer: Timer,
    pub multi_shot_timer: Timer,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            fire_rate_multiplier: 1.0,
            movement_speed_multiplier: 1.0,
            multi_shot: false,
            shield_active: false,
            shield_timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
            rapid_fire_timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            speed_boost_timer: Timer::new(Duration::from_secs(6), TimerMode::Once),
            multi_shot_timer: Timer::new(Duration::from_secs(10), TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct MouseLook {
    pub sensitivity: f32,
}

impl Default for MouseLook {
    fn default() -> Self {
        Self { sensitivity: 5.0 }
    }
}

#[derive(Component)]
pub struct PlayerTurret {
    pub rotation_speed: f32,
    pub current_angle: f32,
    pub side: TurretSide,
}

impl PlayerTurret {
    pub fn new(side: TurretSide) -> Self {
        Self {
            rotation_speed: 8.0,
            current_angle: 0.0,
            side,
        }
    }
}

impl Default for PlayerTurret {
    fn default() -> Self {
        PlayerTurret::new(TurretSide::Left)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TurretSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct TurretParent(pub Entity);

#[derive(Component)]
pub struct Crosshair;
