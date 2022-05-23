use bevy::{prelude::*, gltf::Gltf, utils::HashMap};

use super::AssetsLoading;

#[derive(Default)]
pub struct GltfLoaderPlugin;

impl Plugin for GltfLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GltfHandleStorage>()
            .add_startup_system(load_assets_system);
        }
}

#[derive(Default)]
pub struct GltfHandleStorage(pub HashMap<GltfCollection, Handle<Gltf>>);

#[derive(PartialEq, Eq, Hash)]
pub enum GltfCollection {
    ClawMachine,
    Room,
    HighLander
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<GltfHandleStorage>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    asset_storage.0.insert(GltfCollection::ClawMachine, asset_server.load("models/licensed/claw_machine.glb"));
    asset_storage.0.insert(GltfCollection::Room, asset_server.load("models/licensed/kleeblatt_nosky.glb"));
    asset_storage.0.insert(GltfCollection::HighLander, asset_server.load("models/licensed/caucasian_highlander.glb"));

    assets_loading.add_storage(&asset_storage.0);
}