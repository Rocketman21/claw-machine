use bevy::{prelude::*, app::AppExit};
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;
use strum_macros::Display;

use crate::{
    ui::CursorControl,
    GameState,
    helpers::despawn_with,
    assets::audio::{BackgroundAudioChannel, AudioHandleStorage, AudioCollection},
    gamemodes::gameplay::{Gamemode, GameSettings},
};

use super::controls::{*, menu::CMUIMenu, button::CMUIButton};

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::MainMenu, main_menu_system)
            .add_system(handle_menu_click_system.run_in_state(GameState::MainMenu))
            .add_exit_system(GameState::MainMenu, despawn_with::<CMUIMenu>)
            .add_exit_system(GameState::MainMenu, stop_music);
    }
}

#[derive(PartialEq, Eq, Hash, Display)]
enum MenuButton {
    SpeedGame,
    NumberGame,
    Quit
}

fn main_menu_system(
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut commands: Commands
) {
    if let Some(music) = audio_storage.0.get(&AudioCollection::Background1) {
        audio.play_looped(music.clone());
    }

    commands.spawn()
        .insert(CMUIMenu {
            title: "Menu".to_string(),
            buttons: vec![
                CMUIButton::new(MenuButton::SpeedGame, "Speed game").selected(),
                CMUIButton::new(MenuButton::NumberGame, "Number game"),
                CMUIButton::new(MenuButton::Quit, "Quit"),
            ]
        });
}

fn handle_menu_click_system(
    mut settings: ResMut<GameSettings>,
    mut events: EventReader<ButtonPressEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands
) {
    for event in events.iter() {
        settings.gamemode = if event.0 == MenuButton::SpeedGame.to_string() {
            Gamemode::SpeedGame
        } else if event.0 == MenuButton::NumberGame.to_string() {
            Gamemode::NumberGame
        } else {
            app_exit_events.send(AppExit);

            Gamemode::None
        };

        commands.insert_resource(NextState(GameState::InGame));
    }
}

// TODO into generic system
fn stop_music(audio: Res<AudioChannel<BackgroundAudioChannel>>) {
    audio.stop();
}

// fn main_menu_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut windows: ResMut<Windows>,
//     mut state: ResMut<State<GameState>>,
// ) {
    // if keyboard_input.just_pressed(KeyCode::Escape) {
    //     let window = windows.get_primary_mut().unwrap();

    //     match state.current() {
    //         GameState::Play => {
    //             if let Ok(()) = state.set(GameState::Pause) {
    //                 window.toggle_cursor(false);
    //             }
    //             println!("pausing");
    //         }
    //         GameState::Pause => {
    //             if let Ok(()) = state.set(GameState::Play) {
    //                 window.toggle_cursor(true);
    //             }
    //             println!("playing");
    //         }
    //     }
    // }
// }