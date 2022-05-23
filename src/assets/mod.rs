use std::hash::Hash;

use bevy::{prelude::*, app::PluginGroupBuilder, asset::{LoadState, Asset}, utils::HashMap};
use iyes_loopless::prelude::*;

use crate::GameState;

use self::{gltf::GltfLoaderPlugin, audio::AudioLoaderPlugin};

pub mod gltf;
pub mod audio;

#[derive(Default)]
pub struct AssetLoaderPlugins;
#[derive(Default)]
pub struct AssetLoaderPlugin;
#[derive(Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

impl AssetsLoading {
    fn add_storage<C, A>(&mut self, handle_storage: &HashMap<C, Handle<A>>)
    where
        C: Hash,
        A: Asset,
    {
        handle_storage.iter().for_each(|(_, value)| self.0.push(value.clone_untyped()));
    }
}

impl PluginGroup for AssetLoaderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(AssetLoaderPlugin)
            .add(GltfLoaderPlugin)
            .add(AudioLoaderPlugin);
    }
}

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetsLoading>()
            .add_system(
                check_assets_ready_system.run_in_state(GameState::Loading)
            );
    }
}

fn check_assets_ready_system(
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut commands: Commands
) {
    match server.get_group_load_state(loading.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {
            panic!("Some asset failed to load!")
        }
        LoadState::Loaded => {
            commands.insert_resource(NextState(GameState::MainMenu));

            commands.remove_resource::<AssetsLoading>();
            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}