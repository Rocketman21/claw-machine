use bevy::{prelude::*, app::PluginGroupBuilder};

use self::{gltf::GltfLoaderPlugin, audio::AudioLoaderPlugin};

mod gltf;
mod audio;

#[derive(Default)]
pub struct AssetLoaderPlugins;

impl PluginGroup for AssetLoaderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GltfLoaderPlugin)
            .add(AudioLoaderPlugin);
    }
}