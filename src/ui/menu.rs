use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::prelude::*;

use crate::{ui::CursorControl, GameState, GameMode};

use super::controls::*;

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(main_menu_system.run_in_state(GameState::MainMenu))
            .init_resource::<MenuButtonsStorage>()
            .add_enter_system(GameState::MainMenu, spawn_menu_system);
    }
}

#[derive(Default)]
struct MenuButtonsStorage(HashMap<Entity, MenuButton>);

#[derive(PartialEq, Eq, Hash)]
enum MenuButton {
    SpeedGame,
    NumberGame
}

fn spawn_menu_system(
    controls: Res<Controls>,
    mut menu_buttons: ResMut<MenuButtonsStorage>,
    mut commands: Commands
) {
    commands.spawn()
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
                    parent.spawn_control(controls.button("Speed game")).id.unwrap(),
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
    events: EventReader<ButtonPressEvent>,
    mut commands: Commands
) {
    for event in events.iter() {
        let gamemode = match menu_buttons
            .0
            .get(&event.0)
            .expect("Menu button for given entity does not exist!")
        {
            MenuButton::SpeedGame => GameMode::SpeedGame,
            MenuButton::NumberGame => GameMode::NumberGame
        };

        commands.insert_resource(NextState(GameState::InGame(gamemode)));
    }
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