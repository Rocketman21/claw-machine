use bevy::{prelude::*, gltf::Gltf, utils::HashMap};

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
                        machine
                            .spawn_bundle((Transform::from_xyz(0.0, 0.0, 0.0), GlobalTransform::identity()))
                            .insert(WASDMovement)
                            .with_children(|claw| {
                                claw.spawn_scene(gltf.named_scenes["claw"].clone());
                            });
                    });

                commands.spawn_bundle(PointLightBundle {
                    transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
                    ..Default::default()
                });
            }
        }
    });
}