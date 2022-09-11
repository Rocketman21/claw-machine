use std::f32::consts::PI;

use bevy::{prelude::{*, shape::Capsule}, gltf::Gltf};
use bevy_kira_audio::AudioChannel;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    movement::WASDMovement,
    claw::{
        ClawController,
        ClawObject,
        ClawLift,
        ClawSensor,
        ClawStopper,
        ClawLiftState,
        ClawControllerState,
        ClawString
    },
    constants::{
        COL_GROUP_CLAW,
        COL_GROUP_ALL,
        COL_GROUP_CLAW_STOPPER,
        COL_GROUP_TOY_EJECTION_SHELV,
        COL_GROUP_EJECTED_TOY,
        COL_GROUP_GLASS,
        COL_GROUP_BOTTOM_GLASS
    },
    assets::{
        gltf::{GltfCollection, GltfHandleStorage},
        audio::{AudioCollection, GlassAudioChannel, AudioHandleStorage}
    },
    GameState
};

#[derive(Default)]
pub struct ClawMachinePlugin;

impl Plugin for ClawMachinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlassHitTime>()
            .add_system(glass_hit_system)
            .add_exit_system(GameState::Loading, spawn_claw_machine_system);
    }
}

#[derive(Component)]
pub struct Glass;

#[derive(Default)]
struct GlassHitTime(f64);

const GLASS_SFX: [AudioCollection; 2] = [
    AudioCollection::Glass3,
    AudioCollection::Glass4
];

fn spawn_claw_machine_system(
    assets: Res<Assets<Gltf>>,
    asset_storage: Res<GltfHandleStorage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    if let Some(gltf) = assets.get(asset_storage.0.get(&GltfCollection::ClawMachine).unwrap()) {
        commands.spawn()
            .insert_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransform::identity()))
            .with_children(|machine| {
                machine.spawn_scene(gltf.named_scenes["claw_machine"].clone());
            });

        {// Glass collision
            let thickness = 0.02;
            let [size_x, size_y, size_z] = [0.9, 1.1, 0.9];
            let [x, y, z] = [-0.025, 2.7, -0.05];

            let matrix = [
                [thickness, size_y * 2.0, size_z, x + size_x, y, z],
                [thickness, size_y * 2.0, size_z, x - size_x, y, z],
                [size_x, thickness, size_z, x, y - size_y, z],
                [size_x, thickness, size_z, x, y + size_y, z],
                [size_x, size_y, thickness, x, y, z - size_z],
                [size_x, size_y, thickness, x, y, z + size_z],
            ];

            for (index, coords) in matrix.iter().enumerate() {
                commands.spawn()
                    .insert(Collider::cuboid(coords[0], coords[1], coords[2]))
                    .insert(CollisionGroups::new(
                        if index == 2 {
                            COL_GROUP_BOTTOM_GLASS
                        } else {
                            COL_GROUP_GLASS
                        },
                        COL_GROUP_ALL
                    ))
                    .insert(Friction::new(0.0))
                    .insert(Transform::from_xyz(coords[3], coords[4], coords[5]))
                    .insert(Glass);
            }

            // Inclined shelv for toy to eject
            let shelv = matrix[2];

            for index in 0..2 {
                let mut transform = Transform::from_xyz(shelv[3], shelv[4], shelv[5]);

                commands.spawn()
                    .insert(
                        if index == 0 {
                            transform.translation.z += 0.2;
                            transform.with_rotation(Quat::from_rotation_x(65.0 * PI / 180.0))
                        } else {
                            transform.translation.y -= 0.6;
                            transform
                        }
                    )
                    .insert(Collider::cuboid(shelv[0], shelv[1], shelv[2]))
                    .insert(CollisionGroups::new(COL_GROUP_TOY_EJECTION_SHELV, COL_GROUP_EJECTED_TOY));
            }
        }

        commands.spawn()
            .insert(ClawController(ClawControllerState::Locked))
            .insert_bundle((
                Transform::from_translation(ClawController::BASE_POS.into()),
                GlobalTransform::identity()
            ))
            .insert(Collider::cuboid(0.2, 0.1, 0.2))
            .insert(ColliderMassProperties::Density(140.0))
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED)
            .insert(WASDMovement)
            .with_children(|parent| {
                parent.spawn()
                    .insert(ClawString)
                    .insert_bundle(PbrBundle {
                        transform: Transform::from_xyz(0.001, ClawString::START_HEIGHT, -0.004),
                        mesh: meshes.add(
                             Capsule {
                                radius: ClawString::RADIUS,
                                depth: ClawString::DEPTH,
                                ..default()
                            }.into()
                        ),
                        material: materials.add(Color::BLACK.into()),
                        ..default()
                    });
            });

        let claw_lift = commands.spawn()
            .insert(ClawLift(ClawLiftState::Off))
            .insert_bundle((Transform::from_xyz(0.0, ClawLift::START_HEIGHT, 0.0), GlobalTransform::identity()))

            // not using KinematicPositionBased as it causes a bug with ClawObject remain asleep when lift moves
            .insert(RigidBody::Dynamic)
            .id();

        let spherical_joint = SphericalJointBuilder::new()
            .local_anchor2(Vec3::new(0.0, 0.6, 0.0));

        let claw_object = commands.spawn()
            .insert(ClawObject)
            .insert_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransform::identity()))
            .insert(Collider::cuboid(0.2, 0.2, 0.2))
            .insert(Restitution::coefficient(0.7))
            .insert(CollisionGroups::new(COL_GROUP_CLAW, COL_GROUP_GLASS))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(Damping { linear_damping: 2.0, angular_damping: 2.0 })
            .insert(ImpulseJoint::new(claw_lift, spherical_joint))
            .insert(Velocity::default())
            .with_children(|parent| {
                parent.spawn()
                    .insert_bundle((Transform::from_xyz(-0.53, -3.2, -0.50), GlobalTransform::identity()))
                    .with_children(|claw| { claw.spawn_scene(gltf.named_scenes["claw"].clone()); });

                parent.spawn()
                    .insert(ClawSensor)
                    .insert(Collider::ball(0.1))
                    .insert(CollisionGroups::new(COL_GROUP_ALL, COL_GROUP_ALL - COL_GROUP_CLAW - COL_GROUP_CLAW_STOPPER))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Sensor(true));
            })
            .id();

        commands.spawn()
            .insert(ClawStopper)
            .insert(Collider::cuboid(0.1, 0.05, 0.1))
            .insert(CollisionGroups::new(COL_GROUP_CLAW_STOPPER, COL_GROUP_ALL - COL_GROUP_CLAW))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(RigidBody::Dynamic)
            .insert(ImpulseJoint::new(claw_object, FixedJointBuilder::new().local_anchor1([0.0, 0.1, 0.0].into())))
            .insert(ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)));
    }
}

fn glass_hit_system(
    audio: Res<AudioChannel<GlassAudioChannel>>,
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
                        let hit_force = claw_velocity.linvel.abs().max_element().clamp(0.0, 1.5);

                        if time.seconds_since_startup() - last_hit_time.0 > 0.5 {
                            if let Some(glass_sound) = audio_storage.get_random(&GLASS_SFX) {
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