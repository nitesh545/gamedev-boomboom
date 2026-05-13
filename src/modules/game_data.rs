use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameData {
    pub score: u32,
    pub current_wave: u32,
    pub wave_active: bool,
    pub wave_spawn_timer: Timer,
    pub wave_completion_timer: Timer,
    pub lives: u32,
    pub high_score: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            current_wave: 1,
            wave_active: false,
            wave_spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            wave_completion_timer: Timer::from_seconds(2.0, TimerMode::Once),
            lives: 3,
            high_score: 0,
        }
    }
}
