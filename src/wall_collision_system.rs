use bevy::prelude::*;
use bevy::sprite;
use crate::components::{Ball, BreakoutTile, Player, Speed};
use crate::{AppState, BALL_RADIUS, HEIGHT, PLAYER_DIMENSIONS, WIDTH};

pub fn player_wall_collision_system(mut player_query: Query<&mut Transform, With<Player>>) {
    let mut p_transform = player_query.single_mut();
    if p_transform.translation.x < -(WIDTH / 2.0) + (PLAYER_DIMENSIONS.0 - 70.0) {
        p_transform.translation.x = -(WIDTH / 2.0) + (PLAYER_DIMENSIONS.0 - 70.0);
    }
    if p_transform.translation.x > (WIDTH / 2.0) - (PLAYER_DIMENSIONS.0 - 70.0) {
        p_transform.translation.x = (WIDTH / 2.0) - (PLAYER_DIMENSIONS.0 - 70.0);
    }
}

pub fn ball_wall_collision_system(mut ball_query: Query<(&Transform, &mut Speed), With<Ball>>, mut state: ResMut<State<AppState>>) {
    let (b_transform, mut b_speed) = ball_query.single_mut();
    if b_transform.translation.y > (HEIGHT / 2.0) - (BALL_RADIUS - 25.0) {
        b_speed.y_speed = -b_speed.y_speed;
    }
    if b_transform.translation.y < -(HEIGHT / 2.0) + (BALL_RADIUS + 25.0) {
        match state.current() {
            AppState::InGame => state.set(AppState::End).unwrap(),
            AppState::End => ()
        }
    }
    if b_transform.translation.x < -(WIDTH / 2.0) + (BALL_RADIUS + 175.0) {
        b_speed.x_speed = -b_speed.x_speed;
    }
    if b_transform.translation.x > (WIDTH / 2.0) - (BALL_RADIUS - 175.0) {
        b_speed.x_speed = -b_speed.x_speed;
    }
}

// mut ball_query: Query<(&Transform, &mut Speed), With<Ball>>, mut player_query: Query<&Transform, Without<Ball>>

pub fn ball_paddle_collision_system(mut ball_query: Query<(&Ball, &Transform, &mut Speed)>, mut player_query: Query<(&Player, &Transform)>) {
    let (_ball, b_transform, mut b_speed) = ball_query.single_mut();
    let (_player, p_transform) = player_query.single_mut();
    println!("Player X: {}, Player Y: {}", p_transform.translation.x, p_transform.translation.y);
    if b_transform.translation.x > p_transform.translation.x && b_transform.translation.x < p_transform.translation.x + PLAYER_DIMENSIONS.0 && b_transform.translation.y < -240.0 {
        b_speed.y_speed = -b_speed.y_speed;
    }
}