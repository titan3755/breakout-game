// Module declarations

mod components;
mod setup_system;
mod ball_movement_system;
mod player_movement_system;
mod tiles_collision_system;
mod wall_collision_system;

// Bevy library imports

use bevy::prelude::*;
use bevy::window::PresentMode;

// Other library imports

use bevy_prototype_lyon::prelude::*;

// System & Function imports

use setup_system::*;
use player_movement_system::*;
use ball_movement_system::*;
use wall_collision_system::*;

// Game Constants

pub const WIDTH: f32 = 850.0;
pub const HEIGHT: f32 = 550.0;
pub const APP_TITLE: &str = "Breakout";
pub const RESIZABLE_WINDOW: bool = false;
pub const WINDOW_DECORATIONS: bool = true;
pub const CURSOR_VISIBILITY: bool = false;
pub const BALL_RADIUS: f32 = 15.0;
pub const PLAYER_DIMENSIONS: (f32, f32) = (125.0, 12.0);
pub const PLAYER_SPEED: f32 = 5.0;

// Application entry point

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title: String::from(APP_TITLE),
            resizable: RESIZABLE_WINDOW,
            width: WIDTH,
            height: HEIGHT,
            cursor_visible: CURSOR_VISIBILITY,
            decorations: WINDOW_DECORATIONS,
            present_mode: PresentMode::Fifo,
            position: WindowPosition::Automatic,
            ..Default::default()
        })
        .insert_resource(Msaa {samples: 4})
        .insert_resource(ClearColor{0: Color::rgb(0.0, 181.0, 226.0)})
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(player_movement_system)
        .add_system(ball_movement_system)
        .add_system(player_wall_collision_system)
        .add_system(ball_wall_collision_system)
        .run();
}
