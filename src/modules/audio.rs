#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}
use bevy::audio::{AudioSource, PlaybackMode, Volume};

// Component to track audio entities
#[derive(Component)]
pub struct AudioEffect {
    pub effect_type: AudioEffectType,
    pub duration: Option<f32>, // None for looping sounds
}

#[derive(Clone)]
pub enum AudioEffectType {
    BackgroundMusic,
    EngineLoop,
    LaserShot,
    Explosion,
    EnemyHit,
    MenuClick,
    MenuHover,
}

// Resource to track current audio state
#[derive(Resource)]
pub struct AudioState {
    pub background_music_entity: Option<Entity>,
    pub engine_audio_entity: Option<Entity>,
    pub is_muted: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            background_music_entity: None,
            engine_audio_entity: None,
            is_muted: false,
        }
    }
}

pub fn manage_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
    mut audio_state: ResMut<AudioState>,
) {
    // Start appropriate background music for current state
    let music_file = "audio/music/combat_music_2.ogg";

    let music_entity = commands
        .spawn((
            AudioPlayer::<AudioSource>(asset_server.load(music_file)),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Linear(audio_settings.music_volume * audio_settings.master_volume),
                ..default()
            },
            AudioEffect {
                effect_type: AudioEffectType::BackgroundMusic,
                duration: None,
            },
        ))
        .id();

    audio_state.background_music_entity = Some(music_entity);
}

pub fn audio_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut audio_settings: ResMut<AudioSettings>,
    mut audio_state: ResMut<AudioState>,
    mut commands: Commands,
    audio_entities: Query<Entity, With<AudioEffect>>,
) {
    // Toggle mute with M key
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        audio_state.is_muted = !audio_state.is_muted;

        if audio_state.is_muted {
            // Stop all audio
            for entity in audio_entities.iter() {
                commands.entity(entity).despawn();
            }
            audio_state.background_music_entity = None;
            audio_state.engine_audio_entity = None;
            println!("Audio muted");
        } else {
            println!("Audio unmuted");
        }
    }

    // Volume controls with + and - keys
    if keyboard_input.just_pressed(KeyCode::Equal)
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
    {
        audio_settings.master_volume = (audio_settings.master_volume + 0.1).clamp(0.0, 1.0);
        println!("Volume: {:.1}%", audio_settings.master_volume * 100.0);
    }

    if keyboard_input.just_pressed(KeyCode::Minus)
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
    {
        audio_settings.master_volume = (audio_settings.master_volume - 0.1).clamp(0.0, 1.0);
        println!("Volume: {:.1}%", audio_settings.master_volume * 100.0);
    }
}

pub fn cleanup_finished_audio(
    mut commands: Commands,
    time: Res<Time>,
    mut audio_query: Query<(Entity, &mut AudioEffect)>,
) {
    for (entity, mut audio_effect) in audio_query.iter_mut() {
        if let Some(ref mut duration) = audio_effect.duration {
            *duration -= time.delta_secs();

            if *duration <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn manage_engine_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio_settings: Res<AudioSettings>,
    mut audio_state: ResMut<AudioState>,
    existing_engine: Query<Entity, With<AudioEffect>>,
) {
    let is_moving = keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::KeyS)
        || keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::KeyD)
        || keyboard_input.pressed(KeyCode::ArrowUp)
        || keyboard_input.pressed(KeyCode::ArrowDown)
        || keyboard_input.pressed(KeyCode::ArrowLeft)
        || keyboard_input.pressed(KeyCode::ArrowRight);

    // Start engine audio when moving
    if is_moving && audio_state.engine_audio_entity.is_none() {
        let engine_entity = commands
            .spawn((
                AudioPlayer::<AudioSource>(asset_server.load("audio/sfx/engine_thrust.ogg")),
                PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::Linear(
                        audio_settings.sfx_volume * audio_settings.master_volume * 0.7,
                    ),
                    ..default()
                },
                AudioEffect {
                    effect_type: AudioEffectType::EngineLoop,
                    duration: None,
                },
            ))
            .id();

        audio_state.engine_audio_entity = Some(engine_entity);
    }
    // Stop engine audio when not moving
    else if !is_moving && audio_state.engine_audio_entity.is_some() {
        if let Some(engine_entity) = audio_state.engine_audio_entity {
            if let Ok(entity) = existing_engine.get(engine_entity) {
                commands.entity(entity).despawn();
            }
            audio_state.engine_audio_entity = None;
        }
    }
}

// Helper function to play one-shot sound effects
pub fn play_sound_effect(
    commands: &mut Commands,
    asset_server: &AssetServer,
    audio_settings: &AudioSettings,
    effect_type: AudioEffectType,
    audio_state: &AudioState,
) {
    if audio_state.is_muted {
        return;
    }

    let (audio_file, base_volume, duration) = match effect_type {
        AudioEffectType::LaserShot => ("audio/sfx/player_shoot.ogg", 0.8, 1.0),
        AudioEffectType::Explosion => ("audio/sfx/explosion_small.ogg", 0.75, 1.0),
        AudioEffectType::EnemyHit => ("audio/sfx/enemy_hit.ogg", 0.6, 1.0),
        AudioEffectType::MenuClick => ("audio/ui/button_click.ogg", 0.9, 1.0),
        AudioEffectType::MenuHover => ("audio/ui/button_hover.ogg", 0.9, 1.0),
        _ => return, // Don't play looping sounds as one-shots
    };

    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load(audio_file)),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(
                base_volume * audio_settings.sfx_volume * audio_settings.master_volume,
            ),
            ..default()
        },
        AudioEffect {
            effect_type,
            duration: Some(duration),
        },
    ));
}

// System to play shooting sounds
pub fn play_shooting_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    audio_settings: Res<AudioSettings>,
    audio_state: Res<AudioState>,
) {
    let should_shoot =
        mouse_input.just_pressed(MouseButton::Left) || keyboard_input.just_pressed(KeyCode::Space);

    if should_shoot {
        play_sound_effect(
            &mut commands,
            &asset_server,
            &audio_settings,
            AudioEffectType::LaserShot,
            &audio_state,
        );
    }
}

// System to play menu button click sounds
pub fn play_menu_click_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    audio_settings: Res<AudioSettings>,
    audio_state: Res<AudioState>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            play_sound_effect(
                &mut commands,
                &asset_server,
                &audio_settings,
                AudioEffectType::MenuClick,
                &audio_state,
            );
        }
    }

    for interaction in &interaction_query {
        if *interaction == Interaction::Hovered {
            play_sound_effect(
                &mut commands,
                &asset_server,
                &audio_settings,
                AudioEffectType::MenuHover,
                &audio_state,
            );
        }
    }
}

// Initialize audio resources
pub fn setup_audio_resources(mut commands: Commands) {
    commands.insert_resource(AudioSettings {
        master_volume: 0.7,
        sfx_volume: 0.8,
        music_volume: 0.6,
    });
    commands.insert_resource(AudioState::default());
}

// Alternative: Simple audio playing without tracking
pub fn play_simple_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_file: &str,
    volume: f32,
    looping: bool,
) {
    let audio_file = audio_file.to_string();
    let mode = if looping {
        PlaybackMode::Loop
    } else {
        PlaybackMode::Despawn
    };

    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load(audio_file)),
        PlaybackSettings {
            mode,
            volume: Volume::Linear(volume),
            ..default()
        },
    ));
}

// System to update volume of existing audio when settings change
pub fn update_audio_volumes(
    audio_settings: Res<AudioSettings>,
    mut playback_query: Query<(&mut PlaybackSettings, &AudioEffect)>,
) {
    if audio_settings.is_changed() {
        for (mut playback, audio_effect) in playback_query.iter_mut() {
            let base_volume = match audio_effect.effect_type {
                AudioEffectType::BackgroundMusic => audio_settings.music_volume,
                AudioEffectType::EngineLoop => audio_settings.sfx_volume * 0.7,
                _ => audio_settings.sfx_volume,
            };

            playback.volume = Volume::Linear(base_volume * audio_settings.master_volume);
        }
    }
}
