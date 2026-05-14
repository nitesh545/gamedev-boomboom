#![allow(dead_code)]
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub health: i32,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Chaser,
    Flanker,
    Dasher,
}

#[derive(Component)]
pub struct EnemyBehavior {
    pub target_angle: f32,
    pub behavior_timer: Timer,
    pub dash_cooldown: Timer,
}

impl Default for EnemyBehavior {
    fn default() -> Self {
        Self {
            target_angle: 0.0,
            behavior_timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            dash_cooldown: Timer::new(Duration::from_secs(3), TimerMode::Once),
        }
    }
}
