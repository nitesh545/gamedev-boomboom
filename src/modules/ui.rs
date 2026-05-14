#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MenuButton {
    pub action: MenuAction,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct ScoreText;
