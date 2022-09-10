mod setup_system;
mod components;

use bevy::prelude::*;
use bevy::window::PresentMode;

use setup_system::*;

// Game Constants

pub const WIDTH: f32 = 850.0;
pub const HEIGHT: f32 = 550.0;
pub const APP_TITLE: &str = "Breakout";
pub const RESIZABLE_WINDOW: bool = false;
pub const WINDOW_DECORATIONS: bool = true;
pub const CURSOR_VISIBILITY: bool = false;

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
        .insert_resource(ClearColor{0: Color::rgb(0.0, 181.0, 226.0)})
        .add_startup_system(setup_system)
        .add_plugins(DefaultPlugins)
        .run();
}
