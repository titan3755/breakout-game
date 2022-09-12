use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::components::*;
use crate::{BALL_RADIUS, HEIGHT, WIDTH};

pub fn setup_system(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        center: Vec2 {x: -WIDTH / 2.0 + 250.0, y: -HEIGHT / 2.0 + 250.0}
    };
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK, 2.5),
        },
        Transform::default(),
    )).insert( Ball).insert(Speed {x_speed: -2.0, y_speed: 3.0});
}