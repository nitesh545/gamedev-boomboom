// =============================================================================
// HEALTH SYSTEM - Player Health Management
// =============================================================================

use bevy::prelude::*;

// Component for entities that have health
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            current: health,
            max: health,
        }
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.current = (self.current - damage).max(0.0);
        self.current <= 0.0 // Returns true if health depleted
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn get_health_percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

// System to handle health depletion and lives reduction
pub fn health_to_lives_system(
    mut player_query: Query<
        (&mut Health, &mut crate::modules::lives::Lives),
        With<crate::modules::player::Player>,
    >,
    mut screen_shake: ResMut<crate::modules::camera::ScreenShake>,
) {
    for (mut health, mut lives) in player_query.iter_mut() {
        if !health.is_alive() {
            // Health depleted - lose a life and restore health
            let is_dead = lives.take_damage();

            if !is_dead {
                // Still have lives - restore health
                health.current = health.max;
                screen_shake.trigger(50.0, 0.8);
                println!(
                    "Life lost! Lives remaining: {}, Health restored",
                    lives.current
                );
            } else {
                // No lives left - player is dead
                screen_shake.trigger(100.0, 2.0);
                println!("Player defeated! Game Over.");
            }
        }
    }
}

// Helper function to damage player health (call from collision systems)
pub fn damage_player_health(
    mut player_query: Query<&mut Health, With<crate::modules::player::Player>>,
    damage_amount: f32,
    mut screen_shake: ResMut<crate::modules::camera::ScreenShake>,
) {
    if let Ok(mut health) = player_query.single_mut() {
        health.take_damage(damage_amount);

        // Screen shake based on remaining health
        let health_percent = health.get_health_percentage();
        let shake_intensity = 20.0 + (30.0 * (1.0 - health_percent)); // More shake when low health
        screen_shake.trigger(shake_intensity, 0.3);

        println!(
            "Player took {} damage! Health: {:.1}/{:.1}",
            damage_amount, health.current, health.max
        );
    }
}

// System to reset player health when starting new game
pub fn reset_player_health(
    mut player_query: Query<&mut Health, With<crate::modules::player::Player>>,
) {
    for mut health in player_query.iter_mut() {
        health.current = health.max;
    }
}

pub fn apply_damage_to_player(
    health: &mut Health,
    screen_shake: &mut ResMut<super::ScreenShake>,
    damage: f32,
) {
    health.take_damage(damage);
    let health_percent = health.get_health_percentage();
    let shake_intensity = 20.0 + (30.0 * (1.0 - health_percent));
    screen_shake.trigger(shake_intensity, 0.3);
    println!(
        "Player took {} damage! Health: {:.1}/{:.1}",
        damage, health.current, health.max
    );
}

// =============================================================================
// HEALTH BAR SYSTEM - Visual Health Display Above Player
// =============================================================================

// Component to mark health bar entities
#[derive(Component)]
pub struct HealthBar {
    pub owner: Entity, // Which player this health bar belongs to
}

// Component for the health bar fill (the colored part)
#[derive(Component)]
pub struct HealthBarFill;

// System to spawn health bar when player spawns
pub fn spawn_health_bar(
    mut commands: Commands,
    player_query: Query<
        Entity,
        (
            Added<crate::modules::player::Player>,
            With<crate::modules::health::Health>,
        ),
    >,
) {
    for player_entity in player_query.iter() {
        // Health bar background (dark)
        let _health_bar_bg = commands
            .spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.2, 0.2),
                    custom_size: Some(Vec2::new(60.0, 8.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 40.0, 999.5),
                HealthBar {
                    owner: player_entity,
                },
            ))
            .id();

        // Health bar fill (green/red)
        let _health_bar_fill = commands
            .spawn((
                Sprite {
                    color: Color::srgb(0.2, 0.8, 0.2),       // Green when healthy
                    custom_size: Some(Vec2::new(56.0, 6.0)), // Slightly smaller than bg
                    ..default()
                },
                Transform::from_xyz(0.0, 40.0, 999.8), // Above background
                HealthBarFill,
                HealthBar {
                    owner: player_entity,
                },
            ))
            .id();

        println!("Health bar spawned for player {:?}", player_entity);
    }
}

// System to update health bar position and fill
pub fn update_health_bar(
    player_query: Query<
        (&Transform, &crate::modules::health::Health),
        With<crate::modules::player::Player>,
    >,
    mut health_bar_query: Query<
        (&mut Transform, &HealthBar),
        (
            Without<crate::modules::player::Player>,
            Without<HealthBarFill>,
        ),
    >,
    mut health_fill_query: Query<
        (&mut Transform, &mut Sprite, &HealthBar),
        (With<HealthBarFill>, Without<crate::modules::player::Player>),
    >,
) {
    // Update health bar backgrounds
    for (mut bar_transform, health_bar) in health_bar_query.iter_mut() {
        if let Ok((player_transform, _)) = player_query.get(health_bar.owner) {
            // Position health bar above player
            bar_transform.translation.x = player_transform.translation.x;
            bar_transform.translation.y = player_transform.translation.y + 100.0;
        }
    }

    // Update health bar fill
    for (mut fill_transform, mut fill_sprite, health_bar) in health_fill_query.iter_mut() {
        if let Ok((player_transform, health)) = player_query.get(health_bar.owner) {
            let health_percent = health.get_health_percentage();

            // Position fill above player
            fill_transform.translation.x = player_transform.translation.x;
            fill_transform.translation.y = player_transform.translation.y + 100.0;

            // Update fill width based on health percentage
            let max_width = 56.0;
            let current_width = max_width * health_percent;
            fill_sprite.custom_size = Some(Vec2::new(current_width, 6.0));

            // Update fill color based on health
            fill_sprite.color = if health_percent > 0.6 {
                Color::srgb(0.2, 0.8, 0.2) // Green when healthy
            } else if health_percent > 0.3 {
                Color::srgb(0.8, 0.8, 0.2) // Yellow when moderate
            } else {
                Color::srgb(0.8, 0.2, 0.2) // Red when low
            };
        }
    }
}

// System to cleanup health bars when player is removed
pub fn cleanup_health_bars(
    mut commands: Commands,
    health_bar_query: Query<(Entity, &HealthBar)>,
    player_query: Query<Entity, With<crate::modules::player::Player>>,
) {
    for (bar_entity, health_bar) in health_bar_query.iter() {
        // If the owner player doesn't exist anymore, remove the health bar
        if player_query.get(health_bar.owner).is_err() {
            commands.entity(bar_entity).despawn();
        }
    }
}
