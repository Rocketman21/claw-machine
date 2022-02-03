use bevy::prelude::*;

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WASDMovementSettings { target_index: 0 })
            .add_system(wasd_movement_system);
    }
}

#[derive(Component)]
pub struct WASDMovement;

struct WASDMovementSettings {
    target_index: usize,
}

fn wasd_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&WASDMovement, &mut Transform)>,
    mut settings: ResMut<WASDMovementSettings>,
) {
    let query_iter = query.iter_mut();
    let query_len = query_iter.size_hint();

    for (index, (_, mut transform)) in query_iter.enumerate() {
        if index == settings.target_index {
            const SPEED: f32 = 10.0;
            let distance = SPEED * time.delta_seconds();
            let translation = &mut transform.translation;
    
            if keyboard_input.pressed(KeyCode::W) {
                translation.z -= distance;
            }
            if keyboard_input.pressed(KeyCode::S) {
                translation.z += distance;
            }
            if keyboard_input.pressed(KeyCode::A) {
                translation.x -= distance;
            }
            if keyboard_input.pressed(KeyCode::D) {
                translation.x += distance;
            }
            if keyboard_input.pressed(KeyCode::LShift) {
                translation.y -= distance;
            }
            if keyboard_input.pressed(KeyCode::Space) {
                translation.y += distance;
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        if let Some(length) = query_len.1 {
            if settings.target_index < length - 1 {
                settings.target_index += 1;
            } else {
                settings.target_index = 0;
            }
        }

        println!("settings.target_index: {}", settings.target_index)
    }
}