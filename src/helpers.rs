use std::{hash::Hash, time::Duration};

use bevy::{prelude::*, utils::{HashMap, Instant}};

pub fn despawn_with<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
