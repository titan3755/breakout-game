use bevy::prelude::*;
use std::process::exit;
use crate::AppState;

#[derive(Component)]
struct TextComp;

pub fn gameover_ui_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(ClearColor{0: Color::rgb(0.0, 0.0, 0.0)});
    commands.spawn_bundle(TextBundle {
        text: Text::from_section(
            "     Game Over! Press \"R\" to restart",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                font: asset_server.load("fonts/FiraSans-Medium.ttf")
            },
        ).with_alignment(TextAlignment::CENTER),
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        ..default()
    })
        .insert(TextComp);
}

pub fn gameover_ui_ongoing_system(kbd: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if kbd.pressed(KeyCode::R) {
        match state.current() {
            AppState::End => state.set(AppState::InGame).unwrap(),
            AppState::InGame => ()
        }
    }
    if kbd.pressed(KeyCode::Q) {
        exit(0);
    }
}