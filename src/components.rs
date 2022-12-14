use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Speed {
    pub x_speed: f32,
    pub y_speed: f32
}

#[derive(Component)]
pub enum Direction {
    Static,
    Left,
    Right
}

#[derive(Component)]
pub struct BreakoutTile;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    End,
}