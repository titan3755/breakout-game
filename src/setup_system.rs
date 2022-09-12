use rand::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::components::{Player, Direction, Ball, Speed};
use crate::{BALL_RADIUS, HEIGHT, PLAYER_DIMENSIONS, PLAYER_SPEED, WIDTH};

pub fn setup_system(mut commands: Commands) {
    let rnd_ball_speed: f32 = thread_rng().gen_range(-5.0..5.0);
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
    )).insert( Ball).insert(Speed {x_speed: rnd_ball_speed, y_speed: 10.0 - rnd_ball_speed});
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            custom_size: Some(Vec2 {x: PLAYER_DIMENSIONS.0, y: PLAYER_DIMENSIONS.1}),
            ..default()
        },
        transform: Transform {
            translation: Vec3 {x: WIDTH / 2.0 - (PLAYER_DIMENSIONS.0 + 320.0), y: -240.0, z: 0.0},
            ..default()
        },
        ..default()
    }).insert(Player).insert(Direction::Static).insert(Speed {x_speed: PLAYER_SPEED, y_speed: 0.0});
}