use bevy::prelude::*;

pub fn despawning_system(mut commands: Commands, mut entities: Query<Entity>) {
    for entity_id in entities.iter_mut() {
        commands.entity(entity_id).despawn();
    }
}