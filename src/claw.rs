use bevy::{prelude::*, ecs::event::Events};
use bevy_kira_audio::AudioChannel;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::audio::{AudioHandleStorage, AudioCollection, DropAudioChannel, BackgroundAudioChannel},
    glue::Glue,
    movement::WASDMovement,
    constants::{COL_GROUP_EJECTED_TOY, COL_GROUP_TOY_EJECTION_SHELV, COL_GROUP_GLASS},
    toy::ToySensor,
    GameState
};

#[derive(Default)]
pub struct ClawPlugin;

impl Plugin for ClawPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ReleaseClawEvent>()
            .add_event::<ToyCatchEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(claw_lift_sync_system)
                    .with_system(release_claw_with_keyboard_system)
                    .with_system(claw_lift_activation_system.run_on_event::<ReleaseClawEvent>())
                    .with_system(claw_lift_system)
                    .with_system(claw_return_system)
                    .with_system(claw_manual_control_system)
                    .with_system(claw_stopper_event_manager_system)
                    .into()
            );
    }
}

pub struct ReleaseClawEvent;
pub struct ToyCatchEvent;

pub enum ClawControllerState {
    Locked,
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

const DROP_SFX: [AudioCollection; 6] = [
    AudioCollection::Drop1,
    AudioCollection::Drop2,
    AudioCollection::Drop3,
    AudioCollection::Drop4,
    AudioCollection::Drop5,
    AudioCollection::Drop6,
];

fn claw_lift_sync_system(
    claw_object_query: Query<&Transform, With<ClawController>>,
    mut claw_lift_query: Query<&mut Transform, (With<ClawLift>, Without<ClawController>)>,
) {
    if let (Ok(claw_object_position), Ok(mut claw_lift_position)) = (
        claw_object_query.get_single(),claw_lift_query.get_single_mut()
    ) {
        let mut next_position = claw_object_position.translation;
        next_position.y = claw_lift_position.translation.y;

        claw_lift_position.translation = next_position.into();
    }
}

fn claw_lift_activation_system(
    audio_drop: Res<AudioChannel<DropAudioChannel>>,
    audio_background: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut claw_lift_query: Query<&mut ClawLift>,
    mut claw_controller_query: Query<&mut ClawController>,
) {
    if let (Ok(mut claw_lift), Ok(mut claw_controller)) = (
        claw_lift_query.get_single_mut(), claw_controller_query.get_single_mut()
    ) {
        if let Some(drop_sfx) = audio_storage.get_random(&DROP_SFX) {
            audio_background.stop();
            audio_drop.set_volume(1.5);
            audio_drop.play(drop_sfx.clone());
        }

        claw_controller.0 = ClawControllerState::Locked;
        claw_lift.0 = ClawLiftState::Down;
    }
}

fn release_claw_with_keyboard_system(
    keyboard: Res<Input<KeyCode>>,
    mut events: EventWriter<ReleaseClawEvent>
) {
    if keyboard.just_pressed(KeyCode::Return) {
        events.send(ReleaseClawEvent);
    }
}


fn claw_lift_system(
    time: Res<Time>,
    mut collision_events: EventReader<CollisionEvent>,
    mut toy_catch_events: EventWriter<ToyCatchEvent>,
    mut claw_lift_query: Query<(&mut ClawLift, &mut Transform)>,
    claw_stopper_query: Query<Entity, With<ClawStopper>>,
    claw_sensor_query: Query<Entity, With<ClawSensor>>,
    toy_sensor_query: Query<Entity, With<ToySensor>>,
    mut claw_controller_query: Query<(&mut ClawController, &Transform), Without<ClawLift>>,
    parent_query: Query<&Parent>,
    mut commands: Commands,
) {
    if let Ok((mut claw_lift, mut claw_lift_position)) = claw_lift_query.get_single_mut() {
        let height = claw_lift_position.translation.y;

        match claw_lift.0 {
            ClawLiftState::Down => {
                claw_lift_position.translation.y -= ClawLift::SPEED * time.delta_seconds();

                if let Ok(claw_stopper) = claw_stopper_query.get_single() {
                    for event in collision_events.iter() {
                        // println!("Received collision event: {:?}", event);
                        if let CollisionEvent::Started(entity1, entity2, _) = event {
                            let entities = [entity1, entity2];

                            if let Ok(claw_sensor) = claw_sensor_query.get_single() {
                                for toy_sensor in toy_sensor_query.iter() {
                                    if entities.into_iter().any(|entity| entity == &claw_sensor)
                                        && entities.into_iter().any(|entity| entity == &toy_sensor) {
                                        if let Ok(toy) = parent_query.get(toy_sensor) {
                                            commands.entity(claw_sensor).insert(Glue(toy.0));
                                            toy_catch_events.send(ToyCatchEvent);
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
    glue_query: Query<(Entity, &Glue), With<ClawSensor>>,
    mut commands: Commands,
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
                if let Ok((entity, glue)) = glue_query.get_single() {
                    commands.entity(entity).remove::<Glue>();
                    commands.entity(glue.0).insert(CollisionGroups::new(
                        COL_GROUP_EJECTED_TOY,
                        COL_GROUP_GLASS + COL_GROUP_TOY_EJECTION_SHELV
                    ));
                }

                claw_controller.0 = ClawControllerState::Locked;
                commands.insert_resource(NextState(GameState::ResultsMenu));
            }
        }
    }
}

fn claw_manual_control_system(
    mut claw_controller_query: Query<(Entity, &ClawController), Changed<ClawController>>,
    mut commands: Commands,
) {
    if let Ok((entity, claw_controller)) = claw_controller_query.get_single_mut() {
        if let ClawControllerState::Manual = claw_controller.0 {
            commands.entity(entity).insert(WASDMovement);
        } else {
            commands.entity(entity).remove::<WASDMovement>();
        }
    }
}

fn claw_stopper_event_manager_system(
    mut events: ResMut<Events<CollisionEvent>>,
) {
    events.update();
}