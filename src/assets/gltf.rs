use std::f32::consts::PI;

use bevy::{prelude::*, gltf::Gltf, utils::HashMap};
use bevy_rapier3d::prelude::*;

use crate::{
    movement::WASDMovement,
    claw::{ClawController, ClawObject, ClawLift, ClawSensor, ClawStopper, ClawLiftState},
    constants::{COL_GROUP_CLAW, COL_GROUP_ALL, COL_GROUP_CLAW_STOPPER}
};

#[derive(Default)]
pub struct GltfLoaderPlugin;

impl Plugin for GltfLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GltfHandleStorage>()
            .add_startup_system(load_assets_system)
            .add_system(setup_system)
            .add_system(toy_speed_control_system);
        }
}

#[derive(PartialEq, Eq, Default)]
struct GltfHandleStorage(HashMap<GltfCollection, Handle<Gltf>>);

#[derive(PartialEq, Eq, Hash)]
enum GltfCollection {
    ClawMachine,
    Room,
    HighLander
}

#[derive(Component)]
pub struct Glass;
#[derive(Component)]
pub struct Toy;
#[derive(Component)]
pub struct ToySensor;

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<GltfHandleStorage>,
) {
    asset_storage.0.insert(GltfCollection::ClawMachine, asset_server.load("models/licensed/claw_machine.glb"));
    asset_storage.0.insert(GltfCollection::Room, asset_server.load("models/licensed/kleeblatt_nosky.glb"));
    asset_storage.0.insert(GltfCollection::HighLander, asset_server.load("models/licensed/caucasian_highlander.glb"));
}

fn setup_system(
    mut asset_events: EventReader<AssetEvent<Gltf>>,
    assets: Res<Assets<Gltf>>,
    asset_storage: Res<GltfHandleStorage>,
    mut commands: Commands,
) {
    asset_events.iter().for_each(|event| {
        if let AssetEvent::Created { handle } = event {
            if Some(handle) == asset_storage.0.get(&GltfCollection::Room) {
                let gltf = assets.get(handle).unwrap();

                commands.spawn()
                    .insert_bundle((
                        Transform {
                            scale: [2.2, 2.2, 2.2].into(),
                            rotation: Quat::from_rotation_y(-180.0 * PI / 180.),
                            translation: [-2.2, 0., 5.4].into(),
                            ..Default::default()
                        },
                        GlobalTransform::identity())
                    )
                    .with_children(|parent| {
                        parent.spawn_scene(gltf.scenes[0].clone());
                    });
            }

            if Some(handle) == asset_storage.0.get(&GltfCollection::ClawMachine) {
                let gltf = assets.get(handle).unwrap();

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
                        [thickness, size_y, size_z, x + size_x, y, z],
                        [thickness, size_y, size_z, x - size_x, y, z],
                        [size_x, thickness, size_z, x, y - size_y, z],
                        [size_x, thickness, size_z, x, y + size_y, z],
                        [size_x, size_y, thickness, x, y, z - size_z],
                        [size_x, size_y, thickness, x, y, z + size_z],
                    ];

                    for coords in matrix.iter() {
                        commands
                            .spawn()
                            .insert(Collider::cuboid(coords[0], coords[1], coords[2]))
                            .insert(Transform::from_xyz(coords[3], coords[4], coords[5]))
                            .insert(Glass);
                    }
                }

                commands.spawn()
                    .insert(ClawController)
                    .insert(Collider::cuboid(0.2, 0.1, 0.2))
                    .insert(ColliderMassProperties::Density(140.0))
                    .insert(RigidBody::Dynamic)
                    .insert(Transform::from_xyz(0.0, 3.65, 0.0))
                    .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED)
                    .insert(ExternalImpulse::default())
                    .insert(WASDMovement);

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
                    .insert(CollisionGroups::new(COL_GROUP_CLAW, COL_GROUP_ALL))
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

            if Some(handle) == asset_storage.0.get(&GltfCollection::HighLander)
                // && asset_storage.0.get(&GltfCollection::ClawMachine).is_some()
            {
                let gltf = assets.get(handle).unwrap();
                let size = (0.1, 0.40, 0.25); // true collision is (0.1, 0.44, 0.25)
                let copies = 15;
                let radius = 0.5;

                for index in 1..copies + 1 {
                    let angle = 360.0 / index as f32 * 180.0 / PI;

                    commands.spawn()
                        .insert(Toy)
                        .insert(RigidBody::Dynamic)
                        .insert(Transform {
                            translation: Vec3::new(radius * f32::sin(angle), 2.5, radius * f32::cos(angle)),
                            rotation: Quat::from_rotation_z(angle),
                            ..Default::default()
                        })
                        .insert(GlobalTransform::identity())
                        .insert(Collider::round_cuboid(size.0, size.1, size.2, 0.02))
                        .insert(CollisionGroups::new(COL_GROUP_ALL, COL_GROUP_ALL - COL_GROUP_CLAW))
                        .insert(Velocity::default())
                        .with_children(|parent| {
                            parent.spawn()
                                .insert_bundle((Transform::from_xyz(0.0, -size.1, 0.0), GlobalTransform::identity()))
                                .with_children(|parent| {
                                    parent.spawn_scene(gltf.scenes[0].clone());
                                });
                            
                            parent.spawn()
                                .insert(ToySensor)
                                .insert(Collider::ball(0.2))
                                .insert(ActiveEvents::COLLISION_EVENTS)
                                .insert(Sensor(true));
                        });
                }
            }

            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(2.0, 5.0, 4.0)),
                ..Default::default()
            });
        }
    });
}

fn toy_speed_control_system(mut query: Query<&mut Velocity, With<Toy>>) {
    for mut velocity in query.iter_mut() {
        if velocity.linvel.min_element().abs() > 2.0 {
            velocity.linvel = [0.0, 0.0, 0.0].into();
        }
    }
}