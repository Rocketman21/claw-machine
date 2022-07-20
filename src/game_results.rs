use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;
use strum_macros::Display;

use crate::{
    gamemodes::{speed_game::SpeedGameProgress, number_game::NumberGameProgress},
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
    SpeedGame(SpeedGameProgress),
    NumberGame(NumberGameProgress)
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
        let is_win: bool;
        let win_text: String;

        match results {
            GameResults::SpeedGame(progress) => {
                is_win = progress.toy_caught;
                win_text = format!("{:.2} sec!", &progress.timer.elapsed_secs());
            }
            GameResults::NumberGame(progress) => {
                is_win = progress.toys_caught > 0;
                win_text = format!(
                    "{} toy{}!",
                    &progress.toys_caught,
                    if progress.toys_caught != 1 { "s" } else { "" }
                );
            }
        }

        if is_win {
            audio_storage.0.get(&AudioCollection::Win1)
        } else {
            audio_storage.get_random(&DEFEAT_SFX)
        }.and_then(|sfx| Some(audio.play(sfx.clone())));

        commands.spawn().insert(CMUIMenu {
            title: if is_win {
                win_text
            } else {
                "You lose =(".to_string()
            },
            buttons: vec![
                CMUIButton::new(ResultButtons::MainMenu.to_string(), "Main menu").selected(),
            ]
        });
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
