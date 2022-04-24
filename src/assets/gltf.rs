use std::f32::consts::PI;

use bevy::{prelude::*, gltf::Gltf, utils::HashMap};
use bevy_rapier3d::{prelude::*, na::Point3};

use crate::{movement::WASDMovement, claw::{ClawController, ClawObject, ClawLift}, constants::{COL_GROUP_CLAW, COL_GROUP_ALL}};

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

                commands
                    .spawn_bundle((
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

                commands
                    .spawn_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransform::identity()))
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
                            .spawn_bundle(ColliderBundle {
                                shape: ColliderShape::cuboid(coords[0], coords[1], coords[2]).into(),
                                position: [coords[3], coords[4], coords[5]].into(),
                                ..Default::default()
                            })
                            .insert(ColliderPositionSync::Discrete)
                            .insert(Glass);
                    }
                }

                let claw_controller = commands
                    .spawn_bundle(ColliderBundle {
                        shape: ColliderShape::cuboid(0.2, 0.1, 0.2).into(),
                        mass_properties: ColliderMassProps::Density(140.0).into(),
                        ..Default::default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        position: [0.0, 3.65, 0.0].into(),
                        mass_properties: (RigidBodyMassPropsFlags::TRANSLATION_LOCKED_Y
                            | RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
                        ..Default::default()
                    })
                    .insert(ColliderDebugRender::with_id(0))
                    .insert(ColliderPositionSync::Discrete)
                    .insert(WASDMovement)
                    .insert(ClawController)
                    .id();

                let claw_lift = commands
                    .spawn_bundle(RigidBodyBundle {
                        body_type: RigidBodyType::KinematicPositionBased.into(),
                        // body_type: RigidBodyType::Dynamic.into(),
                        position: [0.0, 3.65, 0.0].into(),
                        ..Default::default()
                    })
                    .insert(ClawLift)
                    .id();

                let claw_object = commands
                    .spawn_bundle(ColliderBundle {
                        shape: ColliderShape::cuboid(0.2, 0.2, 0.2).into(),
                        mass_properties: ColliderMassProps::Density(1.0).into(),
                        material: ColliderMaterial { 
                            restitution: 0.7,
                            ..Default::default()
                        }.into(),
                        flags: ColliderFlags {
                            collision_groups: InteractionGroups::all().with_memberships(COL_GROUP_CLAW),
                            active_events: ActiveEvents::CONTACT_EVENTS,
                            ..Default::default()
                        }.into(),
                        ..Default::default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        damping: RigidBodyDamping { linear_damping: 300.0, angular_damping: 300.0 }.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle((Transform::from_xyz(-0.52, -3.2, -0.52), GlobalTransform::identity()))
                            .with_children(|claw| { claw.spawn_scene(gltf.named_scenes["claw"].clone()); });

                        // Claw stopper
                        // parent // TODO
                        //     .spawn_bundle(ColliderBundle {
                        //         shape: ColliderShape::cuboid(0.2, 0.05, 0.2).into(),
                        //         position: [0.0, -1.0, 0.0].into(),
                        //         ..Default::default()
                        //     })
                        //     .insert(ColliderDebugRender::with_id(1))
                        //     .insert(ColliderPositionSync::Discrete);
                    })
                    // .insert(ColliderDebugRender::with_id(1))
                    .insert(ColliderPositionSync::Discrete)
                    .insert(ClawObject)
                    .id();

                let spherical_joint = SphericalJoint::new()
                    .local_anchor1(Point3::new(0.0, 0.0, 0.0))
                    .local_anchor2(Point3::new(0.0, 0.6, 0.0));
                
                commands.spawn().insert(JointBuilderComponent::new(
                    spherical_joint,
                    claw_lift,
                    claw_object,
                ));
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

                    commands
                        .spawn()
                        .insert_bundle(RigidBodyBundle {
                            position: (
                                Vec3::new(radius * f32::sin(angle), 2.5, radius * f32::cos(angle)),
                                Quat::from_rotation_z(angle)
                            ).into(),
                            ..Default::default()
                        })
                        .insert_bundle(ColliderBundle {
                            shape: ColliderShape::round_cuboid(size.0, size.1, size.2, 0.02).into(),
                            flags: ColliderFlags {
                                collision_groups: InteractionGroups::all().with_filter(COL_GROUP_ALL - COL_GROUP_CLAW),
                                ..Default::default()
                            }.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle((Transform::from_xyz(0.0, -size.1, 0.0), GlobalTransform::identity()))
                                .with_children(|parent| {
                                    parent.spawn_scene(gltf.scenes[0].clone());
                                });
                        })
                        .insert(ColliderPositionSync::Discrete)
                        .insert(Toy);
                }
            }

            commands.spawn_bundle(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(2.0, 5.0, 4.0)),
                ..Default::default()
            });
        }
    });
}

fn toy_speed_control_system(mut query: Query<&mut RigidBodyVelocityComponent, With<Toy>>) {
    for mut velocity in query.iter_mut() {
        if velocity.linvel.camax() > 2.0 {
            velocity.linvel = [0.0, 0.0, 0.0].into();
        } 
    }
}