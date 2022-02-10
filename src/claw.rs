use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::movement::MOVEMENT_KEYS;

#[derive(Default)]
pub struct ClawPlugin;

impl Plugin for ClawPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(claw_lock_system);
    }
}

#[derive(Component)]
pub struct ClawController;

#[derive(Component)]
pub struct PositionLock;

fn claw_lock_system(
    keyboard: Res<Input<KeyCode>>,
    position_lock_query: Query<Entity, With<PositionLock>>,
    claw_controller_query: Query<(Entity, &RigidBodyPositionComponent), With<ClawController>>,
    mut commands: Commands,
) {
    if keyboard.any_just_pressed(MOVEMENT_KEYS.into_iter()) {
        if let Ok(position_lock) = position_lock_query.get_single() {
            commands.entity(position_lock).despawn();
        }
    }

    if keyboard.any_just_released(MOVEMENT_KEYS.into_iter())
        && !keyboard.any_pressed(MOVEMENT_KEYS.into_iter()) {
        if let Ok((claw_controller, position)) = claw_controller_query.get_single() {
            let static_body = commands
                .spawn_bundle(RigidBodyBundle {
                    body_type: RigidBodyType::Static.into(),
                    position: position.position.into(),
                    ..Default::default()
                })
                .id();
    
            commands
                .spawn()
                .insert(JointBuilderComponent::new(
                    FixedJoint::new(),
                    static_body,
                    claw_controller,
                ))
                .insert(PositionLock);
        }
    }
}