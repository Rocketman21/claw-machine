use bevy::{prelude::*, app::PluginGroupBuilder};

use self::gltf::GltfLoaderPlugin;

mod gltf;

#[derive(Default)]
pub struct AssetLoaderPlugins;

impl PluginGroup for AssetLoaderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GltfLoaderPlugin);
    }
}