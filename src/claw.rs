use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{movement::MOVEMENT_KEYS, assets::{audio::{AudioHandleStorage, AudioCollection}, gltf::Glass}};

#[derive(Default)]
pub struct ClawPlugin;

impl Plugin for ClawPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlassHitTime>()
            .add_system(claw_lock_system)
            .add_system(glass_hit_system);
    }
}

#[derive(Component)]
pub struct ClawController;
#[derive(Component)]
pub struct ClawObject;

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
        && !keyboard.any_pressed(MOVEMENT_KEYS.into_iter()) 
    {
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

const GLASS_SOUNDS: [AudioCollection; 3] = [
    AudioCollection::Glass2,
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
    mut contact_events: EventReader<ContactEvent>,
    claw_object_query: Query<(Entity, &RigidBodyVelocityComponent), With<ClawObject>>,
    glass_query: Query<Entity, With<Glass>>,
) {
    if let Ok((claw_object, claw_velocity)) = claw_object_query.get_single() {
        for event in contact_events.iter() {
            if let ContactEvent::Started(handle1, handle2) = event {
                let entities = [handle1.entity(), handle2.entity()];

                if !entities.into_iter().any(|entity| entity == claw_object) { continue; }

                for glass in glass_query.iter() {
                    if entities.into_iter().any(|entity| entity == glass) {
                        let mut rng = rand::thread_rng();
                        let sound = &GLASS_SOUNDS[rng.gen_range(0..GLASS_SOUNDS.len())];
                        let hit_force = claw_velocity.0.linvel.camax() / 20.0;

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