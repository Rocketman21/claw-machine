use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;

use crate::{gamemodes::speed_game::SpeedGameProgress, GameState, assets::audio::{BackgroundAudioChannel, AudioHandleStorage, AudioCollection}, helpers::despawn_with};

#[derive(Default)]
pub struct GameResultsPlugin;

impl Plugin for GameResultsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::GameResults)
                    .with_system(setup_system)
                    .into()
            )
            .add_exit_system(GameState::GameResults, despawn_with::<GameResults>);
            // .add_exit_system(GameState::InGame, exit_system);
    }
}

#[derive(Component)]
pub enum GameResults {
    SpeedGame(SpeedGameProgress)
}

const DEFEAT_SFX: [AudioCollection; 3] = [
    AudioCollection::Defeat1,
    AudioCollection::Defeat2,
    AudioCollection::Defeat3,
];

fn setup_system(
    query: Query<&GameResults, Added<GameResults>>,
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
) {
    if let Ok(results) = query.get_single() {
        match results {
            GameResults::SpeedGame(progress) => {
                if progress.toy_caught {
                    if let Some(sfx) = audio_storage.0.get(&AudioCollection::Win1) {
                        audio.play(sfx.clone());
                    }
                } else {
                    if let Some(sfx) = audio_storage.get_random(&DEFEAT_SFX) {
                        audio.play(sfx.clone());
                    }
                }
                println!("Game results: Time: {}, Toy: {}", progress.timer.elapsed_secs(), progress.toy_caught);
            }
        }
    }
}
