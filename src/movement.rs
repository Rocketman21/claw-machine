use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

const MOVEMENT_KEYS: [KeyCode; 6] = [
    KeyCode::W,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::LShift,
    KeyCode::Space
];

fn wasd_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&WASDMovement, &mut RigidBodyVelocityComponent, &mut RigidBodyMassPropsComponent)>,
    mut settings: ResMut<WASDMovementSettings>,
) {
    let query_iter = query.iter_mut();
    let query_len = query_iter.size_hint();

    for (index, (_, mut velocity, mut mass)) in query_iter.enumerate() {
        if index != settings.target_index { continue; }

        if keyboard.any_pressed(MOVEMENT_KEYS.into_iter()) {
            const SPEED: f32 = 10.0;
            let distance = SPEED * time.delta_seconds();

            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;
    
            if keyboard.pressed(KeyCode::W) {
                z -= distance;
            }
            if keyboard.pressed(KeyCode::S) {
                z += distance;
            }
            if keyboard.pressed(KeyCode::A) {
                x -= distance;
            }
            if keyboard.pressed(KeyCode::D) {
                x += distance;
            }
            if keyboard.pressed(KeyCode::LShift) {
                y -= distance;
            }
            if keyboard.pressed(KeyCode::Space) {
                y += distance;
            }
    
            velocity.apply_impulse(&mass, [x, y, z].into());
        }

        if keyboard.any_just_pressed(MOVEMENT_KEYS.into_iter()) {
            mass.flags = (
                RigidBodyMassPropsFlags::TRANSLATION_LOCKED_Y
                | RigidBodyMassPropsFlags::ROTATION_LOCKED
            ).into();
        }

        if keyboard.any_just_released(MOVEMENT_KEYS.into_iter())
            && !keyboard.any_pressed(MOVEMENT_KEYS.into_iter()) {
            // mass.flags = (
            //     RigidBodyMassPropsFlags::TRANSLATION_LOCKED
            //     | RigidBodyMassPropsFlags::ROTATION_LOCKED
            // ).into();

            // will try to use static joint instead

            velocity.linvel = [0., 0., 0.].into();
        }
    }

    if keyboard.just_pressed(KeyCode::Tab) {
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