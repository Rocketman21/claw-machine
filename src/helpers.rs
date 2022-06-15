use bevy::{prelude::*, ecs::system::Resource};

pub fn despawn_with<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn event_received<T: Resource>(
    events: EventReader<T>,
) -> bool {
    !events.is_empty()
}