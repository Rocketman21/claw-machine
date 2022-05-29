use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ui::CursorControl, GameState};

use super::controls::*;

#[derive(Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(main_menu_system.run_in_state(GameState::MainMenu))
            .add_enter_system(GameState::MainMenu, spawn_menu_system);
    }
}

fn spawn_menu_system(
    controls: Res<Controls>,
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
                    parent.spawn_control(controls.button("Number game"));
                    parent.spawn_control(controls.button("Speed game"));
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