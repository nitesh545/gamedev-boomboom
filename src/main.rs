use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
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
    commands.spawn(
            Camera2d
    );
    commands.spawn(
        (
            Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
            Sprite {
                image: asset_server.load("ground_proto1.png"),
                ..Default::default()
            }
        )
    );

}
