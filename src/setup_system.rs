use bevy::prelude::*;

pub fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}