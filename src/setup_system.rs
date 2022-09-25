use rand::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::components::{Player, Direction, Ball, Speed, BreakoutTile};
use crate::{BALL_RADIUS, HEIGHT, PLAYER_DIMENSIONS, PLAYER_SPEED, TILE_COL_GAP, TILE_COLS, TILE_DIMENSIONS, TILE_ROW_GAP, TILE_ROWS, WIDTH};

pub fn setup_system(mut commands: Commands) {
    let tile_col_spawning_formula = (-WIDTH / 2.0) + 50.0;
    let mut tile_x_distance: f32 = tile_col_spawning_formula;
    let mut tile_y_distance: f32 = 225.0;
    let direction_randomizer: i32 = thread_rng().gen_range(0..=3);
    let mut direction: (f32, f32) = (0.0, 0.0);
    match direction_randomizer {
        0 => {
            direction.0 = 1.0;
            direction.1 = 1.0;
        },
        1 => {
            direction.0 = -1.0;
            direction.1 = 1.0;
        },
        2 => {
            direction.0 = 1.0;
            direction.1 = -1.0;
        },
        3 => {
            direction.0 = -1.0;
            direction.1 = -1.0;
        },
        _ => {
            direction.0 = 1.0;
            direction.1 = 1.0;
        }
    }
    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        center: Vec2 {x: -WIDTH / 2.0 + 250.0, y: -HEIGHT / 2.0 + 250.0}
    };
    commands.insert_resource(ClearColor{0: Color::rgb(0.0, 181.0, 226.0)});
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK, 2.5),
        },
        Transform::default(),
    )).insert( Ball).insert(Speed {x_speed: 5.0 * direction.0, y_speed: 5.0 * direction.1});
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
    for _x in 0..TILE_ROWS {
        for y in 0..TILE_COLS {
            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLUE,
                        custom_size: Some(Vec2::new(TILE_DIMENSIONS.0, TILE_DIMENSIONS.1)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(tile_x_distance as f32, tile_y_distance, 0.0),
                        ..default()
                    },
                    ..default()
                }
            ).insert(BreakoutTile);
            tile_x_distance += TILE_COL_GAP;
        }
        tile_x_distance = tile_col_spawning_formula;
        tile_y_distance -= TILE_ROW_GAP;
    }
}