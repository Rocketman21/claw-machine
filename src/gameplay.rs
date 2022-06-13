use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;
use rand::Rng;

use crate::{
    claw::{ClawController, ClawControllerState},
    GameState, assets::audio::{UiAudioChannel, AudioHandleStorage, AudioCollection, BackgroundAudioChannel},
};

#[derive(Default)]
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .add_enter_system(GameState::InGame, setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(countdown_system)
                    .into()
            );
    }
}

pub enum Gamemode {
    SpeedGame,
    NumberGame
}

#[derive(Default)]
pub struct GameSettings {
    pub gamemode: Option<Gamemode>
}

#[derive(Component)]
struct Countdown(Timer);

const GAMEPLAY_MUSIC: [AudioCollection; 3] = [
    AudioCollection::Gameplay1,
    AudioCollection::Gameplay2,
    AudioCollection::Gameplay3
];

fn setup_system(
    audio: Res<AudioChannel<UiAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut commands: Commands,
) {
    if let Some(countdown) = audio_storage.0.get(&AudioCollection::Countdown) {
        commands.spawn().insert(Countdown(Timer::from_seconds(3.0, false)));

        audio.play(countdown.clone());
    }
}

fn countdown_system(
    time: Res<Time>,
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut query_countdown: Query<&mut Countdown>,
    mut query_claw: Query<&mut ClawController>,
) {
    if let Ok(mut countdown) = query_countdown.get_single_mut() {
        if countdown.0.tick(time.delta()).just_finished() {
            if let Ok(mut claw_controller) = query_claw.get_single_mut() {
                let sound = &GAMEPLAY_MUSIC[rand::thread_rng().gen_range(0..GAMEPLAY_MUSIC.len())];
                
                if let Some(music) = audio_storage.0.get(sound) {
                    audio.play_looped(music.clone());
                }

                claw_controller.0 = ClawControllerState::Manual;
            }
        }
    }
}