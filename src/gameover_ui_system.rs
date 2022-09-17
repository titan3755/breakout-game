use bevy::prelude::*;
use std::process::exit;
use crate::AppState;

#[derive(Component)]
struct TextComp;

pub fn gameover_ui_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor{0: Color::rgb(0.0, 0.0, 0.0)});
    commands.spawn_bundle(
        TextBundle::from_section(
            "Game Over!",
            TextStyle {
                font: asset_server.load("FiraCode-Medium.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
    )
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