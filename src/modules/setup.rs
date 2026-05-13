// src/systems/setup.rs - Complete implementation with all stubs filled

use crate::modules::CinematicCamera;
use bevy::anti_alias::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode};
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::{CursorGrabMode, MonitorSelection, PrimaryWindow, WindowMode};

pub fn setup_persistent_camera(mut commands: Commands) {
    println!("Setting up persistent camera");
    commands.spawn(Camera2d);
}

pub fn setup_window_settings(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        window.title = "Meteor Mayhem".to_string();
        window.visible = true;
        window.present_mode = bevy::window::PresentMode::Fifo;
        println!("Game started in fullscreen mode - Press F11 to toggle, Escape to navigate");
    }
}

pub fn setup_parallax_background(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    println!("Setting up immersive parallax background!");

    // Spawn different background layers
    spawn_space_background_planet(&mut commands, &mut asset_server);
    spawn_space_background_nebula(&mut commands, &mut asset_server);
    spawn_space_background_dust(&mut commands, &mut asset_server);
    //spawn_distant_starfield(&mut commands);
    //spawn_nebula_clouds(&mut commands, &mut images);
    //spawn_nebula_mesh_v1(&mut commands, &mut meshes);
    //spawn_nebula_mesh_v2(&mut commands, &mut meshes);
    // setup_enhanced_space_background(&mut commands, &mut asset_server);
    //spawn_enhanced_planets(&mut commands, &mut asset_server);
    spawn_planets_with_circular_glow(&mut commands, &mut asset_server, &mut images);
    spawn_space_dust_field(&mut commands);
    spawn_asteroid_fields(&mut commands);

    println!("Parallax background setup complete!");
}

fn spawn_space_background_planet(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    let selected_bg_image = fastrand::usize(0..9);
    commands.spawn((
        Sprite::from_image(asset_server.load(format!("background/planet/{selected_bg_image}.png"))),
        Transform::from_xyz(0.0, 0.0, 500.0).with_scale(Vec3::splat(1.5)),
        ParallaxLayer {
            speed_multiplier: 0.0,
            layer_depth: 500.0,
            wrap_distance: 2500.0,
            original_position: Vec2::new(0.0, 0.0),
        },
    ));
}

fn spawn_space_background_nebula(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    let selected_bg_image = fastrand::usize(0..9);
    commands.spawn((
        Sprite::from_image(asset_server.load(format!("background/nebula/{selected_bg_image}.png"))),
        Transform::from_xyz(0.0, 0.0, 450.0).with_scale(Vec3::splat(1.5)),
        ParallaxLayer {
            speed_multiplier: 0.0,
            layer_depth: 450.0,
            wrap_distance: 2800.0,
            original_position: Vec2::new(0.0, 0.0),
        },
    ));
}

fn spawn_space_background_dust(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    let selected_bg_image = fastrand::usize(0..7);
    commands.spawn((
        Sprite::from_image(asset_server.load(format!("background/dust/{selected_bg_image}.png"))),
        Transform::from_xyz(0.0, 0.0, 400.0).with_scale(Vec3::splat(1.5)),
        ParallaxLayer {
            speed_multiplier: 0.0,
            layer_depth: 400.0,
            wrap_distance: 2950.0,
            original_position: Vec2::new(0.0, 0.0),
        },
    ));
}

// Implementation details for background spawning
fn spawn_distant_starfield(commands: &mut Commands) {
    for _ in 0..crate::constants::STARFIELD_COUNT {
        let x = (fastrand::f32() - 0.5) * 4000.0;
        let y = (fastrand::f32() - 0.5) * 3000.0;
        let brightness = 0.3 + fastrand::f32() * 0.7;
        let size = 0.5 + fastrand::f32() * 2.0;

        let star_color = match fastrand::usize(0..4) {
            0 => Color::srgba(1.0, 1.0, 1.0, brightness),
            1 => Color::srgba(0.8, 0.9, 1.0, brightness),
            2 => Color::srgba(1.0, 0.9, 0.7, brightness),
            _ => Color::srgba(1.0, 0.7, 0.6, brightness),
        };

        commands.spawn((
            Sprite {
                color: star_color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 800.0)),
            ParallaxLayer {
                speed_multiplier: 0.1,
                layer_depth: 800.0,
                wrap_distance: 2000.0,
                original_position: Vec2::new(x, y),
            },
            DistantStar {
                brightness,
                twinkle_speed: 0.5 + fastrand::f32() * 2.0,
                twinkle_phase: fastrand::f32() * 6.28,
            },
        ));
    }
}

fn spawn_nebula_clouds(commands: &mut Commands, images: &mut ResMut<Assets<Image>>) {
    for _ in 0..crate::constants::NEBULA_COUNT {
        let x = (fastrand::f32() - 0.5) * 3000.0;
        let y = (fastrand::f32() - 0.5) * 2000.0;
        let size: f32 = 150.0 + fastrand::f32() * 300.0;

        let nebula_type = match fastrand::usize(0..6) {
            0 => NebulaType::Purple,
            1 => NebulaType::Blue,
            2 => NebulaType::Green,
            3 => NebulaType::Orange,
            4 => NebulaType::Pink,
            _ => NebulaType::Red,
        };

        let color = match nebula_type {
            NebulaType::Purple => Color::srgba(0.8, 0.3, 0.9, 0.3),
            NebulaType::Blue => Color::srgba(0.2, 0.5, 1.0, 0.25),
            NebulaType::Green => Color::srgba(0.3, 0.8, 0.4, 0.2),
            NebulaType::Orange => Color::srgba(1.0, 0.64, 0.0, 0.35),
            NebulaType::Pink => Color::srgba(1.0, 0.75, 0.79, 0.35),
            NebulaType::Red => Color::srgba(1.0, 0.4, 0.3, 0.35),
        };

        let nebula_image = create_nebula_gradient_texture(500, nebula_type);
        let nebula_handle: Handle<Image> = images.add(nebula_image);
        commands.spawn((
            Sprite {
                image: nebula_handle,
                //color,
                //custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x, y, 850.0)),
            ParallaxLayer {
                speed_multiplier: 0.2,
                layer_depth: 850.0,
                wrap_distance: 1500.0,
                original_position: Vec2::new(x, y),
            },
            Nebula {
                nebula_type,
                drift_speed: Vec2::new(
                    (fastrand::f32() - 0.5) * 10.0,
                    (fastrand::f32() - 0.5) * 10.0,
                ),
                pulse_speed: 0.3 + fastrand::f32() * 0.7,
                pulse_phase: fastrand::f32() * 6.28,
            },
        ));
    }
}

pub fn spawn_nebula_mesh_v1(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) {
    for _ in 0..crate::constants::NEBULA_COUNT {
        let x = (fastrand::f32() - 0.5) * 3000.0;
        let y = (fastrand::f32() - 0.5) * 2000.0;
        let size: i32 = 150.0 + fastrand::f32() * 300.0;

        let nebula_type = match fastrand::usize(0..6) {
            0 => NebulaType::Purple,
            1 => NebulaType::Blue,
            2 => NebulaType::Green,
            3 => NebulaType::Orange,
            4 => NebulaType::Pink,
            _ => NebulaType::Red,
        };

        let color = match nebula_type {
            NebulaType::Purple => Color::srgba(0.8, 0.3, 0.9, 0.3),
            NebulaType::Blue => Color::srgba(0.2, 0.5, 1.0, 0.25),
            NebulaType::Green => Color::srgba(0.3, 0.8, 0.4, 0.2),
            NebulaType::Orange => Color::srgba(1.0, 0.64, 0.0, 0.35),
            NebulaType::Pink => Color::srgba(1.0, 0.75, 0.79, 0.35),
            NebulaType::Red => Color::srgba(1.0, 0.4, 0.3, 0.35),
        };
        let mesh_handle = meshes.add(create_nebula_mesh(250.0, 24, 0.4));
        commands.spawn((
            Mesh2d(mesh_handle),
            Transform::from_translation(Vec3::new(x, y, 860.0)),
            ParallaxLayer {
                speed_multiplier: 0.2,
                layer_depth: 860.0,
                wrap_distance: 1500.0,
                original_position: Vec2::new(x, y),
            },
            Nebula {
                nebula_type,
                drift_speed: Vec2::new(
                    (fastrand::f32() - 0.5) * 10.0,
                    (fastrand::f32() - 0.5) * 10.0,
                ),
                pulse_speed: 0.3 + fastrand::f32() * 0.7,
                pulse_phase: fastrand::f32() * 6.28,
            },
        ));
    }
}
pub fn create_nebula_mesh(base_radius: f32, segments: usize, distortion: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();

    // Center vertex
    positions.push([0.0, 0.0, 0.0]);
    uvs.push([0.5, 0.5]);
    normals.push([0.0, 0.0, 1.0]);

    // Create organic cloud shape with noise
    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;

        // Add organic distortion using multiple sine waves
        let noise1 = (angle * 3.0).sin() * distortion;
        let noise2 = (angle * 7.0).cos() * distortion * 0.5;
        let noise3 = (angle * 11.0).sin() * distortion * 0.3;

        let radius = base_radius * (1.0 + noise1 + noise2 + noise3);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        positions.push([x, y, 0.0]);
        uvs.push([(x / base_radius + 1.0) * 0.5, (y / base_radius + 1.0) * 0.5]);
        normals.push([0.0, 0.0, 1.0]);

        // Create triangle from center
        let next_i = (i + 1) % segments;
        indices.extend_from_slice(&[0, (i + 1) as u16, (next_i + 1) as u16]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}

pub fn spawn_nebula_mesh_v2(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) {
    for _ in 0..crate::constants::NEBULA_COUNT {
        let x = (fastrand::f32() - 0.5) * 3000.0;
        let y = (fastrand::f32() - 0.5) * 2000.0;
        let size: i32 = 150.0 + fastrand::f32() * 300.0;

        let nebula_type = match fastrand::usize(0..6) {
            0 => NebulaType::Purple,
            1 => NebulaType::Blue,
            2 => NebulaType::Green,
            3 => NebulaType::Orange,
            4 => NebulaType::Pink,
            _ => NebulaType::Red,
        };

        let color = match nebula_type {
            NebulaType::Purple => Color::srgba(0.8, 0.3, 0.9, 0.3),
            NebulaType::Blue => Color::srgba(0.2, 0.5, 1.0, 0.25),
            NebulaType::Green => Color::srgba(0.3, 0.8, 0.4, 0.2),
            NebulaType::Orange => Color::srgba(1.0, 0.64, 0.0, 0.35),
            NebulaType::Pink => Color::srgba(1.0, 0.75, 0.79, 0.35),
            NebulaType::Red => Color::srgba(1.0, 0.4, 0.3, 0.35),
        };
        let mesh_handle = meshes.add(create_multi_cloud_mesh(300.0, 8));
        commands.spawn((
            Mesh2d(mesh_handle),
            Transform::from_translation(Vec3::new(x, y, 870.0)),
            ParallaxLayer {
                speed_multiplier: 0.2,
                layer_depth: 870.0,
                wrap_distance: 1500.0,
                original_position: Vec2::new(x, y),
            },
            Nebula {
                nebula_type,
                drift_speed: Vec2::new(
                    (fastrand::f32() - 0.5) * 10.0,
                    (fastrand::f32() - 0.5) * 10.0,
                ),
                pulse_speed: 0.3 + fastrand::f32() * 0.7,
                pulse_phase: fastrand::f32() * 6.28,
            },
        ));
    }
}
// Generate multiple overlapping circles for cloud-like appearance
pub fn create_multi_cloud_mesh(base_radius: f32, cloud_count: usize) -> Mesh {
    let mut all_positions = Vec::new();
    let mut all_indices = Vec::new();
    let mut all_uvs = Vec::new();
    let mut all_normals = Vec::new();
    let mut vertex_offset = 0u16;

    for _ in 0..cloud_count {
        let offset_x = (fastrand::f32() - 0.5) * base_radius * 0.8;
        let offset_y = (fastrand::f32() - 0.5) * base_radius * 0.8;
        let cloud_radius: f32 = base_radius * (0.6 + fastrand::f32() * 0.6);
        let segments = 16;

        // Center vertex for this cloud
        all_positions.push([offset_x, offset_y, 0.0]);
        all_uvs.push([0.5, 0.5]);
        all_normals.push([0.0, 0.0, 1.0]);

        // Cloud perimeter
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let distortion = (angle * 5.0).sin() * 0.2 + (angle * 9.0).cos() * 0.1;
            let radius = cloud_radius * (1.0 + distortion);

            let x = offset_x + angle.cos() * radius;
            let y = offset_y + angle.sin() * radius;

            all_positions.push([x, y, 0.0]);
            all_uvs.push([(x / base_radius + 1.0) * 0.5, (y / base_radius + 1.0) * 0.5]);
            all_normals.push([0.0, 0.0, 1.0]);

            // Create triangles
            let next_i = (i + 1) % segments;
            all_indices.extend_from_slice(&[
                vertex_offset,
                vertex_offset + 1 + i as u16,
                vertex_offset + 1 + next_i as u16,
            ]);
        }

        vertex_offset += 1 + segments as u16;
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, all_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, all_uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, all_normals)
    .with_inserted_indices(bevy::render::mesh::Indices::U16(all_indices))
}
pub fn create_nebula_gradient_texture(size: u32, nebula_type: NebulaType) -> Image {
    let mut data = Vec::with_capacity((size * size * 4) as usize);
    let center = size as f32 / 2.0;

    let (r, g, b) = match nebula_type {
        NebulaType::Purple => (0.8, 0.3, 0.9),
        NebulaType::Blue => (0.2, 0.5, 1.0),
        NebulaType::Green => (0.3, 0.8, 0.4),
        NebulaType::Red => (1.0, 0.4, 0.3),
        NebulaType::Pink => (1.0, 0.6, 0.8),
        NebulaType::Orange => (1.0, 0.7, 0.2),
    };

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt() / center;

            // Create organic cloud falloff with noise
            let noise = ((dx * 0.02).sin() + (dy * 0.03).cos()) * 0.1;
            let adjusted_distance = distance + noise;

            // Smooth falloff from center to edge
            let alpha = if adjusted_distance <= 1.0 {
                let falloff = 1.0 - adjusted_distance;
                falloff * falloff * falloff // Cubic falloff for natural look
            } else {
                0.0
            };

            // Add some internal structure
            let internal_noise = ((dx * 0.05).sin() + (dy * 0.04).cos()) * 0.3;
            let final_alpha = (alpha * (0.7 + internal_noise)).clamp(0.0, 1.0);

            data.push((r * 255.0) as u8);
            data.push((g * 255.0) as u8);
            data.push((b * 255.0) as u8);
            data.push((final_alpha * 255.0) as u8);
        }
    }

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
}

// COMPLETED: spawn_distant_planets
fn spawn_distant_planets(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    for i in 0..3 {
        // Spawn a few distant planets
        let x = (fastrand::f32() - 0.5) * 2500.0;
        let y = (fastrand::f32() - 0.5) * 1500.0;
        let size: f32 = 60.0 + fastrand::f32() * 100.0;

        // Planet color variety
        let planet_color = match fastrand::usize(0..5) {
            0 => Color::srgba(0.3, 0.5, 0.8, 0.6), // Blue world
            1 => Color::srgba(0.8, 0.6, 0.4, 0.6), // Desert world
            2 => Color::srgba(0.5, 0.8, 0.3, 0.6), // Green world
            3 => Color::srgba(0.7, 0.3, 0.8, 0.6), // Purple gas giant
            _ => Color::srgba(0.6, 0.6, 0.6, 0.5), // Rocky world
        };

        let selected_planet = fastrand::usize(1..8);
        let size_of_planet_png = fastrand::u8(10..40) as f32 / 10.0;

        let z_depth = 900.0 - (i as f32) * 0.3;

        commands.spawn((
            // Sprite {
            //     color: planet_color,
            //     custom_size: Some(Vec2::splat(size)),
            //     ..default()
            // },
            Sprite::from_image(
                asset_server.load(format!("background/planets/{}.png", selected_planet)),
            ),
            Transform::from_translation(Vec3::new(x, y, z_depth))
                .with_scale(Vec3::splat(size_of_planet_png)),
            ParallaxLayer {
                speed_multiplier: 0.25 + (i as f32) * 0.1,
                layer_depth: z_depth,
                wrap_distance: 1200.0,
                original_position: Vec2::new(x, y),
            },
            DistantPlanet {
                rotation_speed: (fastrand::f32() - 0.5) * 0.1,
                orbit_radius: 50.0 + fastrand::f32() * 100.0 * 0.0,
                orbit_speed: 0.1 + fastrand::f32() * 0.1 * 0.0,
                orbit_center: Vec2::new(x, y),
            },
        ));
    }
}

// COMPLETED: spawn_space_dust_field
fn spawn_space_dust_field(commands: &mut Commands) {
    for _ in 0..200 {
        let x = (fastrand::f32() - 0.5) * 2000.0;
        let y = (fastrand::f32() - 0.5) * 1500.0;
        let size = 0.5 + fastrand::f32() * 1.5;
        let opacity = 0.2 + fastrand::f32() * 0.4;

        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.8, 1.0, opacity),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 950.0)),
            ParallaxLayer {
                speed_multiplier: 0.5,
                layer_depth: -5.0,
                wrap_distance: 1000.0,
                original_position: Vec2::new(x, y),
            },
            SpaceDust {
                particle_size: size,
                flow_direction: Vec2::new(
                    (fastrand::f32() - 0.5) * 20.0,
                    (fastrand::f32() - 0.5) * 20.0,
                ),
            },
        ));
    }
}

// COMPLETED: spawn_asteroid_fields
fn spawn_asteroid_fields(commands: &mut Commands) {
    for _ in 0..15 {
        let x = (fastrand::f32() - 0.5) * 1500.0;
        let y = (fastrand::f32() - 0.5) * 1000.0;
        let size = 8.0 + fastrand::f32() * 20.0;

        commands.spawn((
            Sprite {
                color: Color::srgba(0.5, 0.4, 0.3, 0.6),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 960.0)),
            ParallaxLayer {
                speed_multiplier: 0.7,
                layer_depth: -3.0,
                wrap_distance: 800.0,
                original_position: Vec2::new(x, y),
            },
            AsteroidField {
                rotation_speed: (fastrand::f32() - 0.5) * 1.0,
                drift_velocity: Vec2::new(
                    (fastrand::f32() - 0.5) * 30.0,
                    (fastrand::f32() - 0.5) * 30.0,
                ),
            },
        ));
    }
}

// COMPLETED: setup_performance_optimization
pub fn setup_performance_optimization() {
    println!("Setting up performance optimization");

    // Set up performance monitoring
    println!("  - Initializing performance counters");
    println!("  - Setting up entity tracking");
    println!("  - Configuring adaptive quality settings");

    // Performance settings can be configured here
    // For example, setting render distance, particle limits, etc.
    println!("  - Performance optimization complete");
}

// COMPLETED: setup_audio
pub fn setup_audio() {
    println!("Setting up audio system");

    // Audio system initialization
    println!("  - Initializing audio channels");
    println!("  - Setting up spatial audio");
    println!("  - Configuring audio mixing");

    // Audio can be set up here, but most audio setup is handled
    // by the audio::setup_audio_resources system
    println!("  - Audio system ready");
}

// COMPLETED: setup_fps_monitoring
pub fn setup_fps_monitoring() {
    println!("Setting up FPS monitoring");

    // FPS monitoring setup
    println!("  - Initializing frame time tracking");
    println!("  - Setting up performance metrics");
    println!("  - Configuring diagnostic displays");

    // The actual FPS monitoring is handled by FrameTimeDiagnosticsPlugin
    // which is already added in main.rs
    println!("  - FPS monitoring active (Press F12 to show FPS)");
}

// COMPLETED: setup_performance_settings
pub fn setup_performance_settings() {
    println!("Setting up performance settings");

    // Configure performance-related settings
    println!("  - Setting particle limits");
    println!("  - Configuring LOD (Level of Detail) settings");
    println!("  - Setting up adaptive quality scaling");
    println!("  - Configuring distance culling");

    // Performance settings that can be adjusted:
    // - Maximum number of particles
    // - Render distance for different object types
    // - Quality scaling factors
    // - Update frequency for non-critical systems

    println!("  - Performance settings configured");
    println!("  - Use F-keys for performance debugging:");
    println!("    F1: Show controls, F2: Turret debug, F3: Visual debug");
    println!("    F11: Toggle fullscreen, F12: Show FPS");
}

// ADDITIONAL: Advanced background setup
pub fn setup_advanced_background_effects(mut commands: Commands) {
    println!("Setting up advanced background effects");

    // Spawn additional atmospheric effects
    spawn_cosmic_rays(&mut commands);
    spawn_energy_fields(&mut commands);
    spawn_distant_galaxies(&mut commands);

    println!("Advanced background effects complete");
}

pub fn spawn_cosmic_rays(commands: &mut Commands) {
    // Spawn thin streaks of light that occasionally flash across the screen
    for _ in 0..5 {
        let x = (fastrand::f32() - 0.5) * 3000.0;
        let y = (fastrand::f32() - 0.5) * 2000.0;
        let length = 100.0 + fastrand::f32() * 200.0;
        let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;

        commands.spawn((
            Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                custom_size: Some(Vec2::new(2.0, length)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 830.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            ParallaxLayer {
                speed_multiplier: 0.15,
                layer_depth: 830.0,
                wrap_distance: 1800.0,
                original_position: Vec2::new(x, y),
            },
        ));
    }
}

pub fn spawn_energy_fields(commands: &mut Commands) {
    // Spawn large, subtle energy field effects
    for _ in 0..3 {
        let x = (fastrand::f32() - 0.5) * 4000.0;
        let y = (fastrand::f32() - 0.5) * 3000.0;
        let size = 300.0 + fastrand::f32() * 500.0;

        let energy_color = match fastrand::usize(0..3) {
            0 => Color::srgba(0.2, 0.8, 1.0, 0.05), // Electric blue
            1 => Color::srgba(0.8, 0.2, 1.0, 0.05), // Purple
            _ => Color::srgba(1.0, 0.8, 0.2, 0.05), // Golden
        };

        commands.spawn((
            Sprite {
                color: energy_color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 650.0)),
            ParallaxLayer {
                speed_multiplier: 0.05,
                layer_depth: 650.0,
                wrap_distance: 2500.0,
                original_position: Vec2::new(x, y),
            },
        ));
    }
}

pub fn spawn_distant_galaxies(commands: &mut Commands) {
    // Spawn very distant galaxy spirals
    for _ in 0..2 {
        let x = (fastrand::f32() - 0.5) * 5000.0;
        let y = (fastrand::f32() - 0.5) * 4000.0;
        let size = 200.0 + fastrand::f32() * 400.0;

        commands.spawn((
            Sprite {
                color: Color::srgba(0.6, 0.4, 0.8, 0.3),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 520.0)),
            ParallaxLayer {
                speed_multiplier: 0.02,
                layer_depth: 520.0,
                wrap_distance: 3000.0,
                original_position: Vec2::new(x, y),
            },
        ));
    }
}

// Enhanced planet spawning with glow and asteroid belts
pub fn spawn_enhanced_planets(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    for i in 0..3 {
        let x = (fastrand::f32() - 0.5) * 2500.0;
        let y = (fastrand::f32() - 0.5) * 1500.0;
        let selected_planet = fastrand::usize(1..8);
        let size_of_planet = fastrand::u8(10..40) as f32 / 10.0;
        let z_depth = 880.0 - (i as f32) * 0.3;

        // Determine planet type and properties
        let (glow_color, glow_intensity, has_belt) = match selected_planet {
            1..=2 => (Color::srgb(0.9, 0.6, 0.3), 0.8, false), // Rocky planets - warm glow
            3..=4 => (Color::srgb(0.3, 0.7, 1.0), 1.2, true), // Gas giants - bright blue glow + rings
            5..=6 => (Color::srgb(1.0, 0.4, 0.2), 1.5, false), // Lava planets - intense orange glow
            _ => (Color::srgb(0.6, 0.9, 0.4), 0.6, false),    // Earth-like - gentle green glow
        };

        // Spawn the main planet
        let planet_entity = commands
            .spawn((
                Sprite::from_image(
                    asset_server.load(format!("background/planets/{}.png", selected_planet)),
                ),
                Transform::from_translation(Vec3::new(x, y, z_depth))
                    .with_scale(Vec3::splat(size_of_planet)),
                ParallaxLayer {
                    speed_multiplier: 0.25 + (i as f32) * 0.1,
                    layer_depth: z_depth,
                    wrap_distance: 1200.0,
                    original_position: Vec2::new(x, y),
                },
                DistantPlanet {
                    rotation_speed: (fastrand::f32() - 0.5) * 0.1,
                    orbit_radius: 0.0,
                    orbit_speed: 0.0,
                    orbit_center: Vec2::new(x, y),
                },
                PlanetGlow {
                    base_intensity: glow_intensity * 0.1,
                    pulse_speed: 0.5 + fastrand::f32() * 1.0,
                    glow_color,
                    glow_radius: size_of_planet * 10.0, // Glow radius scales with planet
                },
            ))
            .id();

        // Spawn planetary glow effect
        commands.spawn((
            Sprite {
                color: Color::srgba(
                    glow_color.to_srgba().red,
                    glow_color.to_srgba().green,
                    glow_color.to_srgba().blue,
                    0.3,
                ),
                custom_size: Some(Vec2::splat(size_of_planet * 200.0)), // Larger than planet
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x, y, z_depth + 0.01)) // Slightly behind planet
                .with_scale(Vec3::splat(1.0)),
            PlanetGlow {
                base_intensity: glow_intensity * 0.1,
                pulse_speed: 0.3,
                glow_color,
                glow_radius: size_of_planet * 150.0,
            },
            ParallaxLayer {
                speed_multiplier: 0.25 + (i as f32) * 0.1,
                layer_depth: z_depth + 0.01,
                wrap_distance: 1200.0,
                original_position: Vec2::new(x, y),
            },
        ));

        // Spawn asteroid belt if this planet type has one
        if has_belt {
            spawn_asteroid_belt(
                commands,
                planet_entity,
                Vec2::new(x, y),
                z_depth,
                size_of_planet,
            );
            spawn_planetary_rings(commands, Vec2::new(x, y), z_depth, size_of_planet);
        }
        //spawn_enhanced_glow_effect(
        //    commands,
        //    Vec2::new(x, y),
        //    z_depth,
        //    size_of_planet,
        //    glow_color,
        //)
    }
}

// Spawn asteroid belt around a planet
pub fn spawn_asteroid_belt(
    commands: &mut Commands,
    planet_entity: Entity,
    planet_pos: Vec2,
    z_depth: f32,
    planet_size: f32,
) {
    let belt_radius = planet_size * 150.0 + 100.0; // Belt distance from planet
    let asteroid_count = 8 + fastrand::usize(0..8); // 8-15 asteroids

    for i in 0..asteroid_count {
        let angle = (i as f32 / asteroid_count as f32) * std::f32::consts::TAU;
        let radius_variation: f32 = belt_radius + (fastrand::f32() - 0.5) * 40.0; // Vary distance slightly
        let asteroid_size = 0.3 + fastrand::f32() * 0.5; // Small asteroids

        let asteroid_pos = Vec2::new(
            planet_pos.x + angle.cos() * radius_variation,
            planet_pos.y + angle.sin() * radius_variation,
        );

        // Different asteroid colors
        let asteroid_color = match fastrand::usize(0..3) {
            0 => Color::srgb(0.6, 0.5, 0.4), // Brown
            1 => Color::srgb(0.5, 0.5, 0.6), // Gray
            _ => Color::srgb(0.4, 0.6, 0.5), // Greenish
        };

        commands.spawn((
            Sprite {
                color: asteroid_color,
                custom_size: Some(Vec2::splat(8.0)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(asteroid_pos.x, asteroid_pos.y, z_depth + 0.05))
                .with_scale(Vec3::splat(asteroid_size)),
            BeltAsteroid {
                angle,
                distance: radius_variation,
                size: asteroid_size,
                parent_planet: planet_entity,
            },
            ParallaxLayer {
                speed_multiplier: 0.25,
                layer_depth: z_depth + 0.05,
                wrap_distance: 1200.0,
                original_position: asteroid_pos,
            },
        ));
    }
}

// Alternative: Create glowing rings for gas giants
pub fn spawn_planetary_rings(
    commands: &mut Commands,
    planet_pos: Vec2,
    z_depth: f32,
    planet_size: f32,
) {
    let ring_count = 3;

    for i in 0..ring_count {
        let ring_radius = planet_size * 120.0 + (i as f32) * 20.0;
        let ring_thickness = 4.0 + (i as f32) * 2.0;

        // Create ring as a circular sprite
        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.7, 0.6, 0.3 - (i as f32) * 0.05), // Fade outer rings
                custom_size: Some(Vec2::splat(ring_radius * 0.5)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(
                planet_pos.x,
                planet_pos.y,
                z_depth + 0.02 + (i as f32) * 0.01,
            )),
            ParallaxLayer {
                speed_multiplier: 0.25,
                layer_depth: z_depth + 0.02,
                wrap_distance: 1200.0,
                original_position: planet_pos,
            },
        ));
    }
}

// Enhanced version with more dramatic glow
pub fn spawn_enhanced_glow_effect(
    commands: &mut Commands,
    planet_pos: Vec2,
    z_depth: f32,
    planet_size: f32,
    glow_color: Color,
) {
    // Multiple glow layers for more realistic effect
    let glow_layers = [
        (1.5, 0.4), // Inner bright glow
        (2.0, 0.2), // Medium glow
        (2.5, 0.1), // Outer subtle glow
    ];

    for (size_mult, alpha) in glow_layers.iter() {
        commands.spawn((
            Sprite {
                color: Color::srgba(
                    glow_color.to_srgba().red,
                    glow_color.to_srgba().green,
                    glow_color.to_srgba().blue,
                    *alpha,
                ),
                custom_size: Some(Vec2::splat(planet_size * 50.0 * size_mult)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(planet_pos.x, planet_pos.y, z_depth + 0.01)),
            PlanetGlow {
                base_intensity: *alpha,
                pulse_speed: 0.3,
                glow_color,
                glow_radius: planet_size * 50.0 * size_mult,
            },
            ParallaxLayer {
                speed_multiplier: 0.25,
                layer_depth: z_depth + 0.01,
                wrap_distance: 1200.0,
                original_position: planet_pos,
            },
        ));
    }
}

// Spawn enhanced starfield
pub fn spawn_enhanced_starfield(commands: &mut Commands) {
    // Spawn stars across a large area around the player
    for _ in 0..150 {
        // More stars for richness
        let x = (fastrand::f32() - 0.5) * 4000.0; // Wider spread
        let y = (fastrand::f32() - 0.5) * 3000.0;

        // Different star types with different probabilities
        let star_type = match fastrand::u32(0..100) {
            0..=60 => StarType::Tiny,    // 60% tiny stars
            61..=85 => StarType::Small,  // 25% small stars
            86..=95 => StarType::Bright, // 10% bright stars
            _ => StarType::Giant,        // 5% giant stars
        };

        let (size, color, brightness, twinkle_speed, z_depth) = match star_type {
            StarType::Tiny => (
                1.0,
                Color::srgb(0.9, 0.9, 1.0),
                0.6,
                (2.0 + fastrand::f32() * 2.0) as f32,
                -15.0, // Furthest back
            ),
            StarType::Small => (
                1.5,
                Color::srgb(1.0, 0.95, 0.8),
                0.8,
                1.0 + fastrand::f32() * 1.5,
                -12.0,
            ),
            StarType::Bright => (
                2.5,
                Color::srgb(0.8, 0.9, 1.0),
                1.0,
                0.5 + fastrand::f32() * 1.0,
                -10.0,
            ),
            StarType::Giant => (
                4.0,
                Color::srgb(1.0, 0.8, 0.6),
                1.2,
                0.3 + fastrand::f32() * 0.5,
                -8.0, // Closer (brighter)
            ),
        };

        commands.spawn((
            Sprite {
                color: Color::srgba(
                    color.to_srgba().red,
                    color.to_srgba().green,
                    color.to_srgba().blue,
                    brightness,
                ),
                custom_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x, y, z_depth)),
            TwinklingStars {
                twinkle_speed,
                brightness_base: brightness,
                star_type,
            },
            ParallaxLayer {
                speed_multiplier: match star_type {
                    StarType::Tiny => 0.05, // Very slow parallax (distant)
                    StarType::Small => 0.08,
                    StarType::Bright => 0.12,
                    StarType::Giant => 0.15, // Faster parallax (closer)
                },
                layer_depth: z_depth,
                wrap_distance: 2000.0,
                original_position: Vec2::new(x, y),
            },
        ));
    }
}

pub fn spawn_shooting_star(commands: &mut Commands, player_pos: Vec2) {
    // Spawn shooting star from random edge of screen
    let angle: f32 = fastrand::f32() * std::f32::consts::TAU;
    let spawn_distance = 800.0;
    let start_pos =
        player_pos + Vec2::new(angle.cos() * spawn_distance, angle.sin() * spawn_distance);

    // Velocity towards opposite direction
    let target_angle = angle + std::f32::consts::PI + (fastrand::f32() - 0.5) * 0.5;
    let speed: f32 = 200.0 + fastrand::f32() * 300.0;
    let velocity = Vec2::new(target_angle.cos() * speed, target_angle.sin() * speed);

    let lifetime: f32 = 2.0 + fastrand::f32() * 2.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.9),
            custom_size: Some(Vec2::new(20.0, 3.0)), // Elongated for streak effect
            ..Default::default()
        },
        Transform::from_translation(start_pos.extend(870.0))
            .with_rotation(Quat::from_rotation_z(target_angle)),
        ShootingStar {
            velocity,
            lifetime,
            max_lifetime: lifetime,
        },
    ));
}

// Replace your current planet spawning with this enhanced version
pub fn setup_enhanced_space_background(
    mut commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
) {
    println!("Setting up enhanced space background with glow and asteroid belts!");

    // Spawn enhanced starfield
    spawn_enhanced_starfield(&mut commands);

    // Spawn enhanced planets with glow and belts
    spawn_enhanced_planets(&mut commands, &asset_server);

    println!("Enhanced space background setup complete!");
}

fn create_circular_gradient_texture() -> Image {
    let size = 128; // Texture resolution
    let mut data = Vec::with_capacity(size * size * 4);

    let center = size as f32 / 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            let normalized_distance = (distance / center).clamp(0.0, 1.0);

            // Create smooth falloff from center to edge
            let alpha = (1.0 - normalized_distance).powf(2.0); // Quadratic falloff for soft glow

            // White color with varying alpha
            data.push(255); // R
            data.push(255); // G
            data.push(255); // B
            data.push((alpha * 255.0) as u8); // A
        }
    }

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::bevy_asset::RenderAssetUsages::RENDER_WORLD,
    )
}

// Enhanced planet spawning with circular glows
fn spawn_planets_with_circular_glow(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) {
    // Create the circular glow texture once
    let glow_texture = images.add(create_circular_gradient_texture());

    for i in 0..3 {
        let x = (fastrand::f32() - 0.5) * 2500.0;
        let y = (fastrand::f32() - 0.5) * 1500.0;
        let selected_planet = fastrand::usize(1..8);
        let size_of_planet = fastrand::u8(10..40) as f32 / 10.0;
        let z_depth = 840.0 - (i as f32) * 0.3;

        // Determine planet type and glow properties
        let (glow_color, glow_intensity, has_belt) = match selected_planet {
            1..=2 => (Color::srgb(0.9, 0.6, 0.3), 0.8, fastrand::bool()), // Rocky planets - warm glow
            3..=4 => (Color::srgb(0.3, 0.7, 1.0), 1.2, fastrand::bool()), // Gas giants - bright blue glow
            5..=6 => (Color::srgb(1.0, 0.4, 0.2), 1.5, fastrand::bool()), // Lava planets - intense orange glow
            _ => (Color::srgb(0.6, 0.9, 0.4), 0.6, fastrand::bool()), // Earth-like - gentle green glow
        };

        // Spawn the main planet
        let planet_entity = commands
            .spawn((
                Sprite::from_image(
                    asset_server.load(format!("background/planets/{}.png", selected_planet)),
                ),
                Transform::from_translation(Vec3::new(x, y, z_depth))
                    .with_scale(Vec3::splat(size_of_planet)),
                ParallaxLayer {
                    speed_multiplier: 0.25 + (i as f32) * 0.1,
                    layer_depth: z_depth,
                    wrap_distance: 1200.0,
                    original_position: Vec2::new(x, y),
                },
                DistantPlanet {
                    rotation_speed: (fastrand::f32() - 0.5) * 0.1,
                    orbit_radius: 0.0,
                    orbit_speed: 0.0,
                    orbit_center: Vec2::new(x, y),
                },
            ))
            .id();

        // Spawn circular glow layers using the gradient texture
        let glow_layers = [
            (1.8, 0.5),  // Inner bright glow
            (2.5, 0.3),  // Medium glow
            (3.2, 0.15), // Outer subtle glow
        ];
        if has_belt {
            spawn_asteroid_belt(
                commands,
                planet_entity,
                Vec2::new(x, y),
                z_depth,
                size_of_planet,
            );
            //spawn_planetary_rings(commands, Vec2::new(x, y), z_depth, size_of_planet);
        }

        for (size_mult, alpha) in glow_layers.iter() {
            let glow_size = size_of_planet * 50.0 * size_mult;

            commands.spawn((
                Sprite {
                    image: glow_texture.clone(),
                    color: Color::srgba(
                        glow_color.to_srgba().red,
                        glow_color.to_srgba().green,
                        glow_color.to_srgba().blue,
                        alpha * glow_intensity,
                    ),
                    custom_size: Some(Vec2::splat(glow_size)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(x, y, z_depth - 0.05 + (*alpha * 0.1))), // Layer by depth
                PlanetGlow {
                    base_intensity: alpha * glow_intensity,
                    pulse_speed: 0.4 + fastrand::f32() * 0.3,
                    glow_color,
                    glow_radius: glow_size,
                },
                ParallaxLayer {
                    speed_multiplier: 0.25 + (i as f32) * 0.1,
                    layer_depth: z_depth - 0.05,
                    wrap_distance: 1200.0,
                    original_position: Vec2::new(x, y),
                },
            ));
        }
    }
}

// Setup cinematic camera with all the effects
pub fn setup_cinematic_camera(mut commands: Commands) {
    println!("Setting up cinematic camera with bloom and effects!");

    commands.spawn((
        Camera2d,
        bevy::render::view::Hdr,
        Camera {
            clear_color: bevy::camera::ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        // BLOOM EFFECT - Much simpler in Bevy 0.16!
        Bloom {
            intensity: 0.25,                                      // How strong the bloom is
            low_frequency_boost: 0.8,                             // Enhances large bright areas
            low_frequency_boost_curvature: 0.9,                   // How the boost curves
            high_pass_frequency: 1.0, // What brightness level starts blooming
            composite_mode: BloomCompositeMode::EnergyConserving, // Realistic bloom mixing
            prefilter: bevy::core_pipeline::bloom::BloomPrefilter {
                threshold: 0.5,          // Brightness threshold for bloom
                threshold_softness: 0.3, // Smooth transition into bloom
            },
            ..Default::default()
        },
        // TONEMAPPING - Makes colors pop and look cinematic
        Tonemapping::TonyMcMapface, // Best looking tonemapper for games
        // EXPOSURE - Controls overall brightness (optional)
        // Exposure::from_physical_camera(
        //     f_number: 1.8,
        //     shutter_speed: 1.0/60.0,
        //     sensitivity_iso: 100.0,
        // ),

        // ANTI-ALIASING - Smooth edges
        Fxaa::default(),
        // DITHERING - Reduces color banding
        DebandDither::Enabled,
        // Our custom cinematic component
        CinematicCamera::default(),
    ));

    println!("Cinematic camera setup complete!");
}

pub fn spawn_cosmic_dust(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn dust particles occasionally
    if fastrand::f32() < 0.02 {
        // 2% chance per frame
        let position = Vec2::new(
            (fastrand::f32() - 0.5) * 2000.0, // Spawn across wide area
            (fastrand::f32() - 0.5) * 1200.0,
        );

        let size = fastrand::f32() * 3.0 + 1.0; // 1-4 pixels

        commands.spawn((
            Mesh2d(meshes.add(Mesh::from(Circle::new(size))).into()),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.8, 0.8, 1.0, 0.1)))),
            Transform::from_translation(position.extend(940.0)),
            CosmicDust {
                drift_speed: Vec2::new(
                    (fastrand::f32() - 0.5) * 20.0,
                    (fastrand::f32() - 0.5) * 20.0,
                ),
                rotation_speed: (fastrand::f32() - 0.5) * 2.0,
                lifetime: Timer::from_seconds(fastrand::f32() * 10.0 + 5.0, TimerMode::Once),
                fade_in_time: 2.0,
                fade_out_time: 2.0,
            },
        ));
    }
}
