#![allow(dead_code)]
use crate::modules::ui::{MainMenu, MenuAction, MenuButton};
use bevy::prelude::*;

pub fn setup_main_menu(mut commands: Commands) {
    println!("Setting up main menu!");

    spawn_background_stars(&mut commands);
    spawn_floating_asteroids(&mut commands);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Meteor Mayhem"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Survive the Cosmic Storm"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(80.0)),
                    ..default()
                },
            ));

            // Play Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.2)),
                    MenuButton {
                        action: MenuAction::Play,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START GAME"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Quit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.7, 0.2, 0.2)),
                    MenuButton {
                        action: MenuAction::Quit,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("QUIT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("WASD/Arrows: Move • Mouse: Aim • Click/Space: Shoot"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(60.0)),
                    ..default()
                },
            ));
        });

    println!("Main menu setup complete!");
}

fn spawn_background_stars(commands: &mut Commands) {
    for _ in 0..150 {
        let x = (fastrand::f32() - 0.5) * 1600.0;
        let y = (fastrand::f32() - 0.5) * 1200.0;
        let size = 1.0 + fastrand::f32() * 3.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(
                    0.8 + fastrand::f32() * 0.2,
                    0.8 + fastrand::f32() * 0.2,
                    1.0,
                ),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -1.0)),
            MainMenu,
        ));
    }
}

fn spawn_floating_asteroids(commands: &mut Commands) {
    for _ in 0..8 {
        let x = (fastrand::f32() - 0.5) * 1200.0;
        let y = (fastrand::f32() - 0.5) * 800.0;
        let size = 30.0 + fastrand::f32() * 50.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(0.4, 0.3, 0.2),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -0.5)),
            MainMenu,
        ));
    }
}

pub fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenu>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
