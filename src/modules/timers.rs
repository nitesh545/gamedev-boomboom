#![allow(dead_code)]
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct MeteorSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
    pub wave_intensity: f32,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
            wave_intensity: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct CosmicEventTimer;
