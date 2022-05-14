use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{assets::{audio::{AudioHandleStorage, AudioCollection}, gltf::{Glass, ToySensor, Toy}}, glue::Glue, movement::WASDMovement};

#[derive(Default)]
pub struct ClawPlugin;

impl Plugin for ClawPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlassHitTime>()
            .add_system(glass_hit_system)
            .add_system(claw_lift_sync_system)
            .add_system(claw_lift_activation_system)
            .add_system(claw_lift_system)
            .add_system(claw_return_system)
            .add_system(claw_manual_control_system);
        }
}

pub enum ClawControllerState {
    Blocked,
    Manual,
    ReturnToBase(Vec3)
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
    pub const STEP: f32 = 1.2;
}

impl ClawLift {
    pub const START_HEIGHT: f32 = 3.65;
    pub const SPEED: f32 = 1.0;
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
    mut claw_controller_query: Query<&mut ClawController>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        if let (Ok(mut claw_lift), Ok(mut claw_controller)) = (
            claw_lift_query.get_single_mut(), claw_controller_query.get_single_mut()
        ) {
            claw_controller.0 = ClawControllerState::Blocked;
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
    mut claw_controller_query: Query<(&mut ClawController, &Transform), Without<ClawLift>>,
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
                    if let Ok(toy) = glued_toy_query.get_single() {
                        commands.entity(toy).remove::<Glue>();
                    }

                    if let Ok((mut claw_controller, transform)) = claw_controller_query.get_single_mut() {
                        claw_controller.0 = ClawControllerState::ReturnToBase(transform.translation);
                    }
                    
                    claw_lift_position.translation.y = ClawLift::START_HEIGHT;
                    claw_lift.0 = ClawLiftState::Off;
                }
            },
            _ => {}
        }
    }
}

fn claw_return_system(
    time: Res<Time>,
    mut claw_controller_query: Query<(&mut ClawController, &mut Transform)>,
) {
    if let Ok((mut claw_controller, mut transform)) = claw_controller_query.get_single_mut() {
        if let ClawControllerState::ReturnToBase(start_pos) = claw_controller.0 {
            let base = Vec3::from(ClawController::BASE_POS);
            let current_diff = base - transform.translation;
            let start_diff = base - start_pos;
            let step = start_diff / ClawController::STEP * time.delta_seconds();

            if current_diff.abs().max_element() > step.abs().max_element() {
                transform.translation += step;
            } else {
                claw_controller.0 = ClawControllerState::Blocked;
            }
        }
    }
}

fn claw_manual_control_system(
    mut claw_controller_query: Query<(Entity, &ClawController), Changed<ClawController>>,
    mut commands: Commands,
) {
    if let Ok((entity, claw_controller)) = claw_controller_query.get_single_mut() {
        println!("Changed");
        if let ClawControllerState::Manual = claw_controller.0 {
            commands.entity(entity).insert(WASDMovement);
        } else {
            commands.entity(entity).remove::<WASDMovement>();
        }
    }
}