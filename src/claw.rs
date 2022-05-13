use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{movement::MOVEMENT_KEYS, assets::{audio::{AudioHandleStorage, AudioCollection}, gltf::{Glass, ToySensor, Toy}}, glue::Glue};

#[derive(Default)]
pub struct ClawPlugin;

impl Plugin for ClawPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlassHitTime>()
            .add_system(claw_lock_system)
            .add_system(glass_hit_system)
            .add_system(claw_lift_sync_system)
            .add_system(claw_lift_activation_system)
            .add_system(claw_lift_system);
        }
}

pub enum ClawControllerState {
    Off,
    Manual,
    ReturnToBase
}

#[derive(Debug)]
pub enum ClawLiftState {
    Off,
    Down,
    Wait(f32),
    Up
}

#[derive(Component)]
pub struct ClawController(pub ClawControllerState);
#[derive(Component)]
pub struct ClawLift(pub ClawLiftState);
#[derive(Component)]
pub struct ClawObject;
#[derive(Component)]
pub struct ClawSensor;
#[derive(Component)]
pub struct ClawStopper;
#[derive(Component)]
pub struct PositionLock;

impl ClawController {
    pub const BASE_POS: [f32; 3] = [0.54, 3.65, 0.54];
}

impl ClawLift {
    pub const START_HEIGHT: f32 = 3.65;
    pub const SPEED: f32 = 1.0;
}

fn claw_lock_system(
    keyboard: Res<Input<KeyCode>>,
    position_lock_query: Query<Entity, With<PositionLock>>,
    claw_controller_query: Query<(Entity, &Transform), With<ClawController>>,
    mut commands: Commands,
) {
    if keyboard.any_just_pressed(MOVEMENT_KEYS.into_iter()) {
        if let Ok(position_lock) = position_lock_query.get_single() {
            commands.entity(position_lock).despawn();
        }
    }

    if keyboard.any_just_released(MOVEMENT_KEYS.into_iter())
        && !keyboard.any_pressed(MOVEMENT_KEYS.into_iter())
    {
        if let Ok((claw_controller, position)) = claw_controller_query.get_single() {
            let fixed_joint = FixedJointBuilder::new();

            println!("{:?}", position);

            commands.spawn()
                .insert(PositionLock)
                .insert(RigidBody::Fixed)
                .insert(*position)
                .insert(ImpulseJoint::new(claw_controller, fixed_joint));
        }
    }
}

const GLASS_SOUNDS: [AudioCollection; 2] = [
    AudioCollection::Glass3,
    AudioCollection::Glass4
];

#[derive(Default)]
struct GlassHitTime(f64);

fn glass_hit_system(
    audio: Res<Audio>,
    audio_storage: Res<AudioHandleStorage>,
    time: Res<Time>,
    mut last_hit_time: ResMut<GlassHitTime>,
    mut collision_events: EventReader<CollisionEvent>,
    claw_object_query: Query<(Entity, &Velocity), With<ClawObject>>,
    glass_query: Query<Entity, With<Glass>>,
) {
    if let Ok((claw_object, claw_velocity)) = claw_object_query.get_single() {
        for event in collision_events.iter() {
            // println!("Received collision event: {:?}", event);
            if let CollisionEvent::Started(entity1, entity2, _) = event {
                let entities = [entity1, entity2];

                if !entities.into_iter().any(|entity| entity == &claw_object) { continue; }

                for glass in glass_query.iter() {
                    if entities.into_iter().any(|entity| entity == &glass) {
                        let mut rng = rand::thread_rng();
                        let sound = &GLASS_SOUNDS[rng.gen_range(0..GLASS_SOUNDS.len())];
                        let hit_force = Vec2::new(
                            claw_velocity.linvel.min_element().abs(),
                            claw_velocity.linvel.max_element().abs()
                        ).max_element() / 20.0;

                        if hit_force > 0.05 && time.seconds_since_startup() - last_hit_time.0 > 0.5 {
                            if let Some(glass_sound) = audio_storage.0.get(sound) {
                                audio.set_volume(hit_force);
                                audio.play(glass_sound.clone());

                                last_hit_time.0 = time.seconds_since_startup();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn claw_lift_sync_system(
    claw_object_query: Query<&Transform, With<ClawController>>,
    mut claw_lift_query: Query<&mut Transform, (With<ClawLift>, Without<ClawController>)>,
) {
    if let Ok(claw_object_position) = claw_object_query.get_single() {
        if let Ok(mut claw_lift_position) = claw_lift_query.get_single_mut() {
            let mut next_position = claw_object_position.translation;
            next_position.y = claw_lift_position.translation.y;

            claw_lift_position.translation = next_position.into();
        }
    }
}

fn claw_lift_activation_system(
    keyboard: Res<Input<KeyCode>>,
    mut claw_lift_query: Query<&mut ClawLift>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        if let Ok(mut claw_lift) = claw_lift_query.get_single_mut() {
            claw_lift.0 = ClawLiftState::Down;
        }
    }
}

fn claw_lift_system(
    time: Res<Time>,
    mut claw_lift_query: Query<(&mut ClawLift, &mut Transform)>,
    mut collision_events: EventReader<CollisionEvent>,
    claw_stopper_query: Query<Entity, With<ClawStopper>>,
    claw_sensor_query: Query<Entity, With<ClawSensor>>,
    toy_sensor_query: Query<Entity, With<ToySensor>>,
    glued_toy_query: Query<Entity, (With<Toy>, With<Glue>)>,
    parent_query: Query<&Parent>,
    mut commands: Commands
) {
    if let Ok((mut claw_lift, mut claw_lift_position)) = claw_lift_query.get_single_mut() {
        let height = claw_lift_position.translation.y;

        match claw_lift.0 {
            ClawLiftState::Down => {
                claw_lift_position.translation.y -= ClawLift::SPEED * time.delta_seconds();

                if let Ok(claw_stopper) = claw_stopper_query.get_single() {
                    for event in collision_events.iter() {
                        println!("Received collision event: {:?}", event);
                        if let CollisionEvent::Started(entity1, entity2, _) = event {
                            let entities = [entity1, entity2];

                            if let Ok(claw_sensor) = claw_sensor_query.get_single() {
                                for toy_sensor in toy_sensor_query.iter() {
                                    if entities.into_iter().any(|entity| entity == &claw_sensor)
                                        && entities.into_iter().any(|entity| entity == &toy_sensor) {
                                        if let Ok(toy) = parent_query.get(toy_sensor) {
                                            commands.entity(toy.0).insert(Glue(claw_sensor));
                                        }
                                    }
                                }
                            }

                            if entities.into_iter().any(|entity| entity == &claw_stopper) {
                                claw_lift.0 = ClawLiftState::Wait(1.0);

                                break;
                            }
                        }
                    }
                }
            },
            ClawLiftState::Wait(seconds_remain) => {
                if seconds_remain > 0.0 {
                    claw_lift.0 = ClawLiftState::Wait(seconds_remain - time.delta_seconds());
                } else {
                    claw_lift.0 = ClawLiftState::Up;
                }
            }
            ClawLiftState::Up => {
                if height <= ClawLift::START_HEIGHT {
                    claw_lift_position.translation.y += ClawLift::SPEED * time.delta_seconds();
                } else {
                    // if let Ok(toy) = glued_toy_query.get_single() {
                    //     commands.entity(toy).remove::<Glue>();
                    // }

                    claw_lift_position.translation.y = ClawLift::START_HEIGHT;
                    claw_lift.0 = ClawLiftState::Off;
                }
            },
            _ => {}
        }
    }
}