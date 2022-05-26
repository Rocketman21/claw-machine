use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ui::CursorControl, GameState};

use super::controls::{InsertControls, Controls};

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
    windows: Res<Windows>,
    mut commands: Commands
) {
    let window = windows.get_primary().unwrap();
    let (x, y) = (
        100.0,
        window.height() / 4.0
    );

    commands.spawn()
        .insert_bundle(TextBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|menu| {
            for index in 0..=1 {
                menu.spawn_bundle(TextBundle {
                    style: Style {
                        // align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(y - index as f32 * 1.0),
                            left: Val::Px(x - index as f32 * 1.0),
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

            menu.spawn().insert_button(controls.button("Speed game"));
            menu.spawn().insert_button(controls.button("Number game"));
        });

    // commands.spawn().insert_button(controls.button("Speed game"));
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