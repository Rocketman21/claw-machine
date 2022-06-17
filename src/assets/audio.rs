use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{AudioSource, AudioApp};
use rand::Rng;

use super::AssetsLoading;

#[derive(Default)]
pub struct AudioLoaderPlugin;

pub struct BackgroundAudioChannel;
pub struct GlassAudioChannel;
pub struct DropAudioChannel;
pub struct UiAudioChannel;

impl Plugin for AudioLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioHandleStorage>()
            .add_audio_channel::<BackgroundAudioChannel>()
            .add_audio_channel::<GlassAudioChannel>()
            .add_audio_channel::<DropAudioChannel>()
            .add_audio_channel::<UiAudioChannel>()
            .add_startup_system(load_assets_system);
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct AudioHandleStorage(pub HashMap<AudioCollection, Handle<AudioSource>>);

impl AudioHandleStorage {
    pub fn get_random(&self, collection: &[AudioCollection]) -> Option<&Handle<AudioSource>> {
        let sound = &collection[rand::thread_rng().gen_range(0..collection.len())];

        self.0.get(sound)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum AudioCollection {
    Background1,

    Countdown,

    Glass3,
    Glass4,

    Drop1,
    Drop2,
    Drop3,
    Drop4,
    Drop5,
    Drop6,

    Gameplay1,
    Gameplay2,
    Gameplay3,

    Win1,

    Defeat1,
    Defeat2,
    Defeat3,

    ButtonPress
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AudioHandleStorage>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    asset_storage.0.insert(AudioCollection::Background1, asset_server.load("audio/background1.ogg"));

    asset_storage.0.insert(AudioCollection::Countdown, asset_server.load("audio/countdown.ogg"));

    asset_storage.0.insert(AudioCollection::Glass3, asset_server.load("audio/glass3.ogg"));
    asset_storage.0.insert(AudioCollection::Glass4, asset_server.load("audio/glass4.ogg"));

    asset_storage.0.insert(AudioCollection::Drop1, asset_server.load("audio/drop1.ogg"));
    asset_storage.0.insert(AudioCollection::Drop2, asset_server.load("audio/drop2.ogg"));
    asset_storage.0.insert(AudioCollection::Drop3, asset_server.load("audio/drop3.ogg"));
    asset_storage.0.insert(AudioCollection::Drop4, asset_server.load("audio/drop4.ogg"));
    asset_storage.0.insert(AudioCollection::Drop5, asset_server.load("audio/drop5.ogg"));
    asset_storage.0.insert(AudioCollection::Drop6, asset_server.load("audio/drop6.ogg"));

    asset_storage.0.insert(AudioCollection::Gameplay1, asset_server.load("audio/gameplay1.ogg"));
    asset_storage.0.insert(AudioCollection::Gameplay2, asset_server.load("audio/gameplay2.ogg"));
    asset_storage.0.insert(AudioCollection::Gameplay3, asset_server.load("audio/gameplay3.ogg"));

    asset_storage.0.insert(AudioCollection::Win1, asset_server.load("audio/win1.ogg"));

    asset_storage.0.insert(AudioCollection::Defeat1, asset_server.load("audio/defeat1.ogg"));
    asset_storage.0.insert(AudioCollection::Defeat2, asset_server.load("audio/defeat2.ogg"));
    asset_storage.0.insert(AudioCollection::Defeat3, asset_server.load("audio/defeat3.ogg"));

    asset_storage.0.insert(AudioCollection::ButtonPress, asset_server.load("audio/button.ogg"));

    assets_loading.add_storage(&asset_storage.0);
}