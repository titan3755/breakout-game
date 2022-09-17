// Module declarations

mod components;
mod setup_system;
mod ball_movement_system;
mod player_movement_system;
mod tiles_collision_system;
mod wall_collision_system;
mod despawning_system;
mod gameover_ui_system;

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
use despawning_system::*;
use gameover_ui_system::*;
use components::AppState;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state(AppState::InGame)
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_movement_system)
                .with_system(ball_movement_system)
                .with_system(player_wall_collision_system)
                .with_system(ball_wall_collision_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(despawning_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::End)
                .with_system(gameover_ui_setup_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::End)
                .with_system(gameover_ui_ongoing_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::End)
                .with_system(despawning_system)
        )
        .run();
}

// .add_startup_system(setup_system)
// .add_system(player_movement_system)
// .add_system(ball_movement_system)
// .add_system(player_wall_collision_system)
// .add_system(ball_wall_collision_system)
