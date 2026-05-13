use crate::modules::enemies::Enemy;
use crate::modules::player::Player;
use bevy::camera::Exposure;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

#[derive(Component)]
pub struct CinematicCamera {
    pub bloom_intensity: f32,
    pub exposure: f32,
    pub base_position: Vec3, // ✅ ADD: Store the base camera position
    pub target_zoom: f32,
    pub current_zoom: f32,
}

impl Default for CinematicCamera {
    fn default() -> Self {
        Self {
            bloom_intensity: 0.3,
            exposure: 0.0,
            base_position: Vec3::new(0.0, 0.0, 1000.0), // ✅ Default camera position
            target_zoom: 1.0,
            current_zoom: 1.0,
        }
    }
}

// ✅ SIMPLIFIED: Single shake resource
#[derive(Resource, Default, Clone)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
    pub enabled: bool,
}

impl ScreenShake {
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.timer = 0.0;
        self.enabled = true;
        println!(
            "🔥 Screen shake triggered! Intensity: {:.2}, Duration: {:.2}",
            intensity, duration
        );
    }

    pub fn is_active(&self) -> bool {
        self.enabled && self.timer < self.duration
    }

    pub fn get_shake_offset(&self) -> Vec3 {
        if !self.is_active() {
            return Vec3::ZERO;
        }

        let progress = (self.timer / self.duration).clamp(0.0, 1.0);
        let fade = (1.0 - progress).powi(2); // Quadratic fade for smooth ending
        let current_intensity = self.intensity * fade;

        // Generate smooth random shake
        let time_seed = (self.timer * 50.0) as u64;
        let shake_x = (fastrand::Rng::with_seed(time_seed).f32() - 0.5) * 2.0 * current_intensity;
        let shake_y = (fastrand::Rng::with_seed(time_seed).f32() - 0.5) * 2.0 * current_intensity;

        Vec3::new(shake_x, shake_y, 0.0)
    }
}

// ✅ UNIFIED: Single camera shake system (replaces both old systems)
pub fn unified_camera_shake_system(
    mut camera_query: Query<(&mut Transform, &mut CinematicCamera), With<Camera>>,
    mut screen_shake: ResMut<ScreenShake>,
    time: Res<Time>,
) {
    // Update shake timer
    if screen_shake.is_active() {
        screen_shake.timer += time.delta_secs();

        if screen_shake.timer >= screen_shake.duration {
            screen_shake.enabled = false;
            println!("✅ Screen shake completed");
        }
    }

    // Apply shake to camera
    for (mut transform, cinematic) in camera_query.iter_mut() {
        if screen_shake.is_active() {
            let shake_offset = screen_shake.get_shake_offset();
            transform.translation = cinematic.base_position + shake_offset;

            // Debug output (every 10 frames to avoid spam)
            if (screen_shake.timer * 60.0) as i32 % 10 == 0 {
                println!(
                    "📱 Active shake: offset=({:.2}, {:.2}), intensity={:.2}",
                    shake_offset.x,
                    shake_offset.y,
                    screen_shake.intensity * (1.0 - screen_shake.timer / screen_shake.duration)
                );
            }
        } else {
            // Reset to base position when not shaking
            transform.translation = cinematic.base_position;
        }
    }
}

// ✅ DEBUG: Test shake with T/Y keys
pub fn debug_screen_shake_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        screen_shake.trigger(20.0, 0.3); // Light shake
        println!("🔧 DEBUG: Light shake triggered (T key)");
    }

    if keyboard.just_pressed(KeyCode::KeyY) {
        screen_shake.trigger(50.0, 0.8); // Heavy shake
        println!("🔧 DEBUG: Heavy shake triggered (Y key)");
    }
}

// ✅ HELPER: Easy shake trigger function for other systems
pub fn trigger_screen_shake_easy(
    screen_shake: &mut ResMut<ScreenShake>,
    intensity: f32,
    duration: f32,
) {
    screen_shake.trigger(intensity, duration);
}

// ✅ UPDATED: Dynamic bloom system (unchanged, but improved logging)
pub fn dynamic_bloom_system(
    mut camera_query: Query<(&mut Bloom, &mut CinematicCamera)>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    for (mut bloom, mut cinematic) in camera_query.iter_mut() {
        let mut target_intensity = 0.25;

        if let Ok(player_transform) = player_query.single() {
            let player_pos = player_transform.translation.truncate();
            let nearby_enemies = enemy_query
                .iter()
                .filter(|enemy_transform| {
                    let distance = player_pos.distance(enemy_transform.translation.truncate());
                    distance < 300.0
                })
                .count();

            target_intensity += nearby_enemies as f32 * 0.05;
        }

        cinematic.bloom_intensity = cinematic
            .bloom_intensity
            .lerp(target_intensity, time.delta_secs() * 2.0);
        bloom.intensity = cinematic.bloom_intensity.clamp(0.1, 0.8);
    }
}

// ✅ KEEP: Your existing bloom debug controls
pub fn bloom_debug_controls(
    mut camera_query: Query<&mut Bloom>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut bloom) = camera_query.single_mut() {
        if keyboard.just_pressed(KeyCode::Digit1) {
            bloom.intensity = (bloom.intensity - 0.05).max(0.0);
            println!("Bloom intensity: {:.2}", bloom.intensity);
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            bloom.intensity = (bloom.intensity + 0.05).min(1.0);
            println!("Bloom intensity: {:.2}", bloom.intensity);
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            bloom.prefilter.threshold = (bloom.prefilter.threshold - 0.1).max(0.0);
            println!("Bloom threshold: {:.2}", bloom.prefilter.threshold);
        }
        if keyboard.just_pressed(KeyCode::Digit4) {
            bloom.prefilter.threshold = (bloom.prefilter.threshold + 0.1).min(2.0);
            println!("Bloom threshold: {:.2}", bloom.prefilter.threshold);
        }
    }
}

// ✅ KEEP: Your existing dynamic exposure system
pub fn dynamic_exposure_system(
    mut camera_query: Query<(&mut Exposure, &CinematicCamera)>,
    time: Res<Time>,
) {
    for (mut exposure, _cinematic) in camera_query.iter_mut() {
        let time_factor = (time.elapsed_secs() * 0.1).sin() * 0.1;
        let base_exposure = 0.0;
        exposure.ev100 = base_exposure + time_factor;
    }
}

// =============================================================================
// INTEGRATION EXAMPLES - Add shake to your collision systems
// =============================================================================

// ✅ Example: Add to asteroid collision system
pub fn example_asteroid_collision_with_shake(
    // ... your existing collision parameters
    mut screen_shake: ResMut<ScreenShake>,
) {
    // When asteroid is destroyed:
    screen_shake.trigger(15.0, 0.2); // Medium shake

    // When large asteroid splits:
    screen_shake.trigger(25.0, 0.4); // Bigger shake
}

// ✅ Example: Add to enemy collision system
pub fn example_enemy_collision_with_shake(
    // ... your existing collision parameters
    mut screen_shake: ResMut<ScreenShake>,
) {
    // Different intensities for different enemies:
    screen_shake.trigger(10.0, 0.15); // Light shake for small enemies
    // screen_shake.trigger(20.0, 0.3);  // Medium shake for medium enemies
    // screen_shake.trigger(60.0, 1.0);  // Massive shake for boss defeats
}

// ✅ Example: Add to player damage system
pub fn example_player_damage_with_shake(
    // ... your existing damage parameters
    mut screen_shake: ResMut<ScreenShake>,
) {
    // Heavy shake when player takes damage:
    screen_shake.trigger(40.0, 0.6);
}

// =============================================================================
// CAMERA SETUP UPDATE - Make sure your camera has CinematicCamera component
// =============================================================================
pub fn setup_space_game_camera_updated(mut commands: Commands) {
    let camera_position = Vec3::new(0.0, 0.0, 1000.0);

    commands.spawn((
        Camera2d::default(),
        Transform::from_translation(camera_position),
        bevy::render::view::Hdr,
        Camera {
            ..Default::default()
        },
        //Camera2dBundle {
        //    transform: Transform::from_translation(camera_position),
        //    camera: Camera {
        //        hdr: true, // Enable HDR for bloom
        //        ..default()
        //    },
        //    ..default()
        //},
        Bloom {
            intensity: 0.3,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            prefilter: bevy::post_process::bloom::BloomPrefilter {
                threshold: 0.8,
                threshold_softness: 0.5,
            },
            composite_mode: bevy::post_process::bloom::BloomCompositeMode::Additive,
            ..Default::default()
        },
        CinematicCamera {
            base_position: camera_position, // ✅ Important: Store base position
            ..default()
        },
    ));

    println!("📷 Updated camera setup with unified shake support");
}

// =============================================================================
// REMOVE OLD SYSTEMS - Delete these from your main.rs systems list:
// =============================================================================
/*
❌ REMOVE THESE OLD SYSTEMS:
- camera::camera_shake_system
- camera::update_screen_shake
- Any references to the old ScreenShake resource fields

✅ REPLACE WITH:
- camera::unified_camera_shake_system
- camera::debug_screen_shake_input (for testing)
*/
