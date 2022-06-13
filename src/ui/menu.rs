use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;

use crate::{ui::CursorControl, GameState, helpers::despawn_with, assets::audio::{BackgroundAudioChannel, AudioHandleStorage, AudioCollection}, gameplay::{GameSettings, Gamemode}};

use super::controls::*;

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(main_menu_system.run_in_state(GameState::MainMenu))
            .init_resource::<MenuButtonsStorage>()
            .add_enter_system(GameState::MainMenu, spawn_menu_system)
            .add_system(handle_menu_click_system.run_in_state(GameState::MainMenu))
            .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
            .add_exit_system(GameState::MainMenu, stop_music);
    }
}

#[derive(Default)]
struct MenuButtonsStorage(HashMap<Entity, MenuButton>);

#[derive(PartialEq, Eq, Hash)]
enum MenuButton {
    SpeedGame,
    NumberGame
}

#[derive(Component)]
struct MainMenu;

fn spawn_menu_system(
    controls: Res<Controls>,
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut menu_buttons: ResMut<MenuButtonsStorage>,
    mut commands: Commands
) {
    if let Some(music) = audio_storage.0.get(&AudioCollection::Background1) {
        audio.play_looped(music.clone());
    }

    commands.spawn()
        .insert(MainMenu)
        .insert_bundle(TextBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size { width: Val::Percent(50.0), height: Val::Percent(100.0) },
                ..default()
            },
            ..default()
        })
        .with_children(|menu| {
            menu.spawn_bundle(TextBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    margin: Rect { left: Val::Px(50.0), ..default() },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                menu_buttons.0.insert(
                    parent.spawn_control(controls.button("Number game")).id.unwrap(),
                    MenuButton::NumberGame
                );
                menu_buttons.0.insert(
                    parent.spawn_control(controls.button("Speed game").selected()).id.unwrap(),
                    MenuButton::SpeedGame
                );
            });

            menu.spawn_bundle(TextBundle {
                style: Style {
                    size: Size { width: Val::Px(250.0), height: Val::Px(65.0) },
                    margin: Rect { bottom: Val::Px(100.0), ..default() },
                    ..default()
                },
                ..default()
            })
                .with_children(|parent| {
                    for index in 0..=1 {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: Rect {
                                    top: Val::Px(-index as f32),
                                    left: Val::Px(-index as f32),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text::with_section(
                                "Menu",
                                TextStyle {
                                    font: controls.header_font.clone(),
                                    font_size: 100.0,
                                    color: if index == 0 {
                                        Color::GRAY
                                    } else {
                                        Color::WHITE
                                    },
                                },
                                default()
                            ),
                            ..default()
                        });
                    }
                });
        });
}

fn handle_menu_click_system(
    menu_buttons: Res<MenuButtonsStorage>,
    mut game_settings: ResMut<GameSettings>,
    mut events: EventReader<ButtonPressEvent>,
    mut commands: Commands
) {
    for event in events.iter() {
        if let Some(menu_button) = menu_buttons.0.get(&event.0) {
            game_settings.gamemode = Some(match menu_button {
                MenuButton::SpeedGame => Gamemode::SpeedGame,
                MenuButton::NumberGame => Gamemode::NumberGame
            });

            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}

// TODO into generic system
fn stop_music(audio: Res<AudioChannel<BackgroundAudioChannel>>) {
    audio.stop();
}

fn main_menu_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut state: ResMut<State<GameState>>,
) {
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
}