// =============================================================================
// LIVES SYSTEM - Player Health Management
// =============================================================================
#![allow(dead_code)]
use bevy::prelude::*;

// Component for entities that have lives
#[derive(Component)]
pub struct Lives {
    pub current: i32,
    pub max: i32,
}

impl Lives {
    pub fn new(lives: i32) -> Self {
        Self {
            current: lives,
            max: lives,
        }
    }

    pub fn take_damage(&mut self) -> bool {
        self.current -= 1;
        self.current <= 0 // Returns true if dead
    }

    pub fn heal(&mut self) {
        self.current = (self.current + 1).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}

// System to handle player taking damage
pub fn player_damage_system(
    mut player_query: Query<(Entity, &mut Lives), With<crate::modules::player::Player>>,
    mut _screen_shake: ResMut<crate::modules::camera::ScreenShake>,
) {
    for (_player_entity, mut _lives) in player_query.iter_mut() {
        // This system will be called from collision systems
        // when player takes damage
    }
}

// System to check for game over
pub fn check_game_over(player_query: Query<&Lives, With<crate::modules::player::Player>>) {
    if let Ok(lives) = player_query.single() {
        if !lives.is_alive() {
            // TODO: implement this functionality.
            println!("It's not implemented yet.");
            unimplemented!();
        }
    }
}

// Helper function to damage player (call from collision systems)
pub fn damage_player(
    mut player_query: Query<&mut Lives, With<crate::modules::player::Player>>,
    mut screen_shake: ResMut<crate::modules::camera::ScreenShake>,
) -> bool {
    if let Ok(mut lives) = player_query.single_mut() {
        let is_dead = lives.take_damage();

        // Screen shake on damage
        let shake_intensity = if is_dead { 80.0 } else { 40.0 };
        let shake_duration = if is_dead { 1.5 } else { 0.6 };
        screen_shake.trigger(shake_intensity, shake_duration);

        println!("Player damaged! Lives remaining: {}", lives.current);
        return is_dead;
    }
    false
}

// System to reset player lives when starting new game
pub fn reset_player_lives(
    mut player_query: Query<&mut Lives, With<crate::modules::player::Player>>,
) {
    for mut lives in player_query.iter_mut() {
        lives.current = lives.max;
    }
}
