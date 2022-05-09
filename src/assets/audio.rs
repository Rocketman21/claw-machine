use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::AudioSource;

#[derive(Default)]
pub struct AudioLoaderPlugin;

impl Plugin for AudioLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioHandleStorage>()
            .add_startup_system(load_assets_system);
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct AudioHandleStorage(pub HashMap<AudioCollection, Handle<AudioSource>>);

#[derive(PartialEq, Eq, Hash)]
pub enum AudioCollection {
    Glass3,
    Glass4,
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AudioHandleStorage>,
) {
    asset_storage.0.insert(AudioCollection::Glass3, asset_server.load("audio/glass3.ogg"));
    asset_storage.0.insert(AudioCollection::Glass4, asset_server.load("audio/glass4.ogg"));
}