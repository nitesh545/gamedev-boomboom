#![allow(clippy::all)]
// More pedantic/stylistic lints
#![allow(clippy::pedantic)]
// Experimental lints (may have false positives)
#![allow(clippy::nursery)]
// Cargo-related lints
#![allow(clippy::cargo)]
// Performance-related lints
#![allow(clippy::perf)]
// Complexity lints
#![allow(clippy::complexity)]
// Style lints
#![allow(clippy::style)]
// Correctness lints (usually errors)
#![allow(clippy::correctness)]
// Suspicious code patterns
#![allow(clippy::suspicious)]

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
// use bevy::image::{ImageFilterMode, ImageSamplerDescriptor};
// use std::time::Duration;
// use vleue_kinetoscope::AnimatedImagePlugin;

mod modules;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_ground)
        .add_plugins(PlayerPlugin)
        .run();
}

pub fn load_ground(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
        Sprite {
            image: asset_server.load("ground_proto1.png"),
            ..Default::default()
        },
    ));
}
