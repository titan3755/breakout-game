use bevy::prelude::*;
use crate::components::{Ball, Speed};

pub fn ball_movement_system(mut query: Query<(&mut Transform, &Speed), With<Ball>>) {
    let (mut transform, speed) = query.single_mut();
    transform.translation.x += speed.x_speed;
    transform.translation.y += speed.y_speed;
}