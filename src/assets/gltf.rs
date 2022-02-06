use bevy::{prelude::*, gltf::Gltf, utils::HashMap};
use bevy_rapier3d::{prelude::*, na::Point3};

use crate::movement::WASDMovement;

#[derive(Default)]
pub struct GltfLoaderPlugin;

impl Plugin for GltfLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetHandleStorage>()
            .add_startup_system(load_assets_system)
            .add_system(setup_system);
        }
}

#[derive(PartialEq, Eq, Default)]
struct AssetHandleStorage(HashMap<GltfCollection, Handle<Gltf>>);

#[derive(PartialEq, Eq, Hash)]
enum GltfCollection {
    ClawMachine,
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AssetHandleStorage>,
) {
    asset_storage.0.insert(GltfCollection::ClawMachine, asset_server.load("models/claw_machine.glb"));
}

fn setup_system(
    mut asset_events: EventReader<AssetEvent<Gltf>>,
    assets: Res<Assets<Gltf>>,
    asset_storage: Res<AssetHandleStorage>,
    mut commands: Commands,
) {
    asset_events.iter().for_each(|event| {
        if let AssetEvent::Created { handle } = event {
            if Some(handle) == asset_storage.0.get(&GltfCollection::ClawMachine) {
                let gltf = assets.get(handle).unwrap();

                commands
                    .spawn_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransform::identity()))
                    .with_children(|machine| {
                        machine.spawn_scene(gltf.named_scenes["claw_machine"].clone());

                    });

                {
                    let thickness = 0.01;
                    let size = [0.9, 1.2, 0.9];
                    let center = [-0.025, 2.5, -0.05];

                    let matrix = [
                        ((size[0], thickness, size[2]), (center[0], center[1] / 2., center[2])),
                        // (size[0], thickness, size[2], center[0]),
                        // (size[0], thickness, size[2], center[0]),
                        // (size[0], thickness, size[2], center[0]),
                        // (size[0], thickness, size[2], center[0]),
                        // (size[0], thickness, size[2], center[0]),
                    ];

                    for coords in matrix.iter() {
                        commands
                        .spawn_bundle(ColliderBundle {
                            shape: ColliderShape::cuboid(coords.0.0, coords.0.1, coords.0.2).into(),
                            // mass_properties: ColliderMassProps::Density(140.0).into(),
                            ..Default::default()
                        })
                        .insert_bundle(RigidBodyBundle {
                            body_type: RigidBodyType::Static.into(),
                            position: [coords.1.0, coords.1.1, coords.1.2].into(),
                            ..Default::default()
                        })
                        .insert(ColliderPositionSync::Discrete)
                        .insert(ColliderDebugRender::with_id(2));
                    }
                }
                // commands
                //     .spawn_bundle(ColliderBundle {
                //         shape: ColliderShape::cuboid(0.9, 1.2, 0.9).into(),
                //         // mass_properties: ColliderMassProps::Density(140.0).into(),
                //         ..Default::default()
                //     })
                //     .insert_bundle(RigidBodyBundle {
                //         body_type: RigidBodyType::Static.into(),
                //         position: [-0.025, 2.5, -0.05].into(),
                //         ..Default::default()
                //     })
                //     .insert(ColliderPositionSync::Discrete);
                //     // .insert(ColliderDebugRender::with_id(2));

                let claw_controller = commands
                    // .spawn_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransfoDynamicrm::identity()))
                    .spawn_bundle(ColliderBundle {
                        shape: ColliderShape::cuboid(0.2, 0.1, 0.2).into(),
                        mass_properties: ColliderMassProps::Density(140.0).into(),
                        ..Default::default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        body_type: RigidBodyType::Dynamic.into(),
                        position: [0.0, 3.65, 0.0].into(),
                        mass_properties: (RigidBodyMassPropsFlags::TRANSLATION_LOCKED_Y
                            | RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
                        // damping: RigidBodyDamping { linear_damping: 10000.0, angular_damping: 1000.0 }.into(),
                        ..Default::default()
                    })
                    .insert(ColliderDebugRender::with_id(0))
                    .insert(ColliderPositionSync::Discrete)
                    .insert(WASDMovement)
                    .id();

                let claw_object = commands
                    .spawn_bundle(ColliderBundle {
                        shape: ColliderShape::ball(0.2).into(),
                        mass_properties: ColliderMassProps::Density(50.0).into(),
                        ..Default::default()
                    })
                    .insert_bundle(RigidBodyBundle {
                        damping: RigidBodyDamping { linear_damping: 100.0, angular_damping: 0.0 }.into(),
                        ..Default::default()
                    })
                    .insert(ColliderDebugRender::with_id(1))
                    .insert(ColliderPositionSync::Discrete)
                    // .with_children(|claw| { claw.spawn_scene(gltf.named_scenes["claw"].clone()); })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle((Transform {
                                scale: [5.0, 5.0, 5.0].into(),
                                translation: [-2.6, -15.5, -2.5].into(),
                                ..Default::default()
                            }, GlobalTransform::identity()))
                            .with_children(|claw| { claw.spawn_scene(gltf.named_scenes["claw"].clone()); });
                    })
                    .id();

                let joint = SphericalJoint::new().local_anchor2(Point3::new(0.0, 0.6, 0.0));
                
                commands.spawn_bundle((JointBuilderComponent::new(
                    joint,
                    claw_controller,
                    claw_object,
                ),));

                commands.spawn_bundle(PointLightBundle {
                    transform: Transform::from_translation(Vec3::new(2.0, 5.0, 4.0)),
                    ..Default::default()
                });
            }
        }
    });
}