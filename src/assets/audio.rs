use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{AudioSource, AudioApp};
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

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

#[derive(PartialEq, Eq, Hash, Display, EnumIter, Clone, Copy)]
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

    Button
}

fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AudioHandleStorage>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    for audio in AudioCollection::iter() {
        asset_storage.0.insert(
            audio,
            asset_server.load(format!("audio/{}.ogg", audio.to_string().to_lowercase()).as_str())
        );
    }

    assets_loading.add_storage(&asset_storage.0);
}