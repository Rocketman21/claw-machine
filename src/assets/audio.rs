use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{AudioSource, Audio};

#[derive(Default)]
pub struct AudioLoaderPlugin;

impl Plugin for AudioLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetHandleStorage>()
            .add_startup_system(load_assets_system);
            // .add_system(setup_system);
    }
}

#[derive(PartialEq, Eq, Default)]
struct AssetHandleStorage(HashMap<AudioCollection, Handle<AudioSource>>);

#[derive(PartialEq, Eq, Hash)]
enum AudioCollection {
    Glass1,
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AssetHandleStorage>,
    audio: Res<Audio>,
) {
    asset_storage.0.insert(AudioCollection::Glass1, asset_server.load("audio/glass1.ogg"));
    
    audio.set_volume(0.1);
    audio.play(asset_server.load("audio/glass1.ogg"));
}