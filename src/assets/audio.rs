use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{AudioSource, AudioApp};

use super::AssetsLoading;

#[derive(Default)]
pub struct AudioLoaderPlugin;

pub struct BackgroundAudioChannel;
pub struct GlassAudioChannel;
pub struct DropAudioChannel;

impl Plugin for AudioLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioHandleStorage>()
            .add_audio_channel::<BackgroundAudioChannel>()
            .add_audio_channel::<GlassAudioChannel>()
            .add_audio_channel::<DropAudioChannel>()
            .add_startup_system(load_assets_system);
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct AudioHandleStorage(pub HashMap<AudioCollection, Handle<AudioSource>>);

#[derive(PartialEq, Eq, Hash)]
pub enum AudioCollection {
    Glass3,
    Glass4,

    Drop1,
    Drop2,
    Drop3,
    Drop4,
    Drop5,
    Drop6,
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AudioHandleStorage>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    asset_storage.0.insert(AudioCollection::Glass3, asset_server.load("audio/glass3.ogg"));
    asset_storage.0.insert(AudioCollection::Glass4, asset_server.load("audio/glass4.ogg"));

    asset_storage.0.insert(AudioCollection::Drop1, asset_server.load("audio/drop1.ogg"));
    asset_storage.0.insert(AudioCollection::Drop2, asset_server.load("audio/drop2.ogg"));
    asset_storage.0.insert(AudioCollection::Drop3, asset_server.load("audio/drop3.ogg"));
    asset_storage.0.insert(AudioCollection::Drop4, asset_server.load("audio/drop4.ogg"));
    asset_storage.0.insert(AudioCollection::Drop5, asset_server.load("audio/drop5.ogg"));
    asset_storage.0.insert(AudioCollection::Drop6, asset_server.load("audio/drop6.ogg"));

    assets_loading.add_storage(&asset_storage.0);
}