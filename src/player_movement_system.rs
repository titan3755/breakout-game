use bevy::prelude::*;
use crate::components::{Player, Direction, Speed};

pub fn player_movement_system(kbd: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Direction, &Speed), With<Player>>) {
    let (mut transform, mut direction, speed) = query.single_mut();
    match *direction {
        Direction::Left => transform.translation.x -= speed.x_speed,
        Direction::Right => transform.translation.x += speed.x_speed,
        _ => ()
    }

    if kbd.pressed(KeyCode::A) || kbd.pressed(KeyCode::D) {

        if kbd.pressed(KeyCode::A) {
            *direction = Direction::Left;
        }

        if kbd.pressed(KeyCode::D) {
            *direction = Direction::Right;
        }

    }
    else {
        *direction = Direction::Static;
    }
}