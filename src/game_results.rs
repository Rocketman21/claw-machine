use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;
use strum_macros::Display;

use crate::{
    gamemodes::speed_game::SpeedGameProgress,
    GameState,
    assets::audio::{BackgroundAudioChannel, AudioHandleStorage, AudioCollection},
    helpers::despawn_with,
    ui::controls::{menu::CMUIMenu, button::CMUIButton, ButtonPressEvent}
};

#[derive(Default)]
pub struct GameResultsPlugin;

impl Plugin for GameResultsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::GameResults)
                    .with_system(setup_system)
                    .with_system(handle_menu_click_system)
                    .into()
            )
            .add_exit_system(GameState::GameResults, despawn_with::<GameResults>)
            .add_exit_system(GameState::GameResults, despawn_with::<CMUIMenu>);
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

#[derive(Display)]
enum ResultButtons {
    MainMenu
}

fn setup_system(
    query: Query<&GameResults, Added<GameResults>>,
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut commands: Commands,
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

                commands.spawn()
                    .insert(CMUIMenu {
                        title: if progress.toy_caught {
                            format!("Caught: {:.2} sec!", &progress.timer.elapsed_secs())
                        } else {
                            "You lose =(".to_string()
                        },
                        buttons: vec![
                            CMUIButton::new(ResultButtons::MainMenu.to_string(), "Main menu >").selected(),
                        ]
                    });
                // println!("Game results: Time: {}, Toy: {}", progress.timer.elapsed_secs(), progress.toy_caught);
            }
        }
    }
}

fn handle_menu_click_system(
    mut events: EventReader<ButtonPressEvent>,
    mut commands: Commands
) {
    for event in events.iter() {
        if event.0 == ResultButtons::MainMenu.to_string() {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
    }
}
