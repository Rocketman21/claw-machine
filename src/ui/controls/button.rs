use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

use crate::{assets::audio::{UiAudioChannel, AudioHandleStorage, AudioCollection}, constants::PURPLE_COLOR};

use super::{Controls, SpawnedControl};

#[derive(Component, Clone)]
pub struct CMUIButton {
    pub key: String,
    pub text: &'static str,
    pub is_selected_by_default: bool
}

impl CMUIButton {
    const COLOR_NORMAL: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    const COLOR_ACTIVE: Color = PURPLE_COLOR;
    const ITERACTION_KEYS: [KeyCode; 3] = [
        KeyCode::W,
        KeyCode::S,
        KeyCode::Return
    ];

    pub fn new<T: ToString>(key: T, text: &'static str) -> Self {
        CMUIButton { key: key.to_string(), text, is_selected_by_default: false }
    } 

    pub fn selected(mut self) -> Self {
        self.is_selected_by_default = true;

        self
    }
}

#[derive(Default)]
pub struct ButtonState {
    selected: Option<Entity>,
    prev_selected: Option<Entity>
}

#[derive(Component)]
pub struct SelectedByDefault(bool);

pub struct ButtonPressEvent(pub String);

pub fn button_spawner_system(
    controls: Res<Controls>,
    buttons: Query<(Entity, &CMUIButton), Without<SpawnedControl<CMUIButton>>>,
    mut commands: Commands
) {
    for (entity, component) in buttons.iter() {
        commands.entity(entity)
            .insert(SpawnedControl::<CMUIButton>::new())
            .with_children(|button| {
                button.spawn()
                    .insert(SelectedByDefault(component.is_selected_by_default))
                    .insert_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(280.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect { top: Val::Px(10.0), ..default() },
                            ..default()
                        },
                        color: if component.is_selected_by_default {
                            CMUIButton::COLOR_ACTIVE.into()
                        } else {
                            CMUIButton::COLOR_NORMAL.into()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for index in 0..=1 {
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        top: Val::Px(14.0 - (index * 2) as f32),
                                        left: Val::Px(20.0 - (index * 2) as f32),
                                        ..default()
                                    },
                                    ..default()
                                },
                                text: Text::with_section(
                                    component.text.clone(),
                                    TextStyle {
                                        font: controls.font.clone(),
                                        font_size: 40.0,
                                        color: if index == 0 {
                                            PURPLE_COLOR
                                        } else {
                                            Color::ANTIQUE_WHITE
                                        },
                                    },
                                    default(),
                                ),
                                ..default()
                            });
                        }
                    });
                });
    }
}

pub fn handle_interaction_system(
    mut event: EventWriter<ButtonPressEvent>,
    mut state: ResMut<ButtonState>,
    mut interaction_query: Query<(Entity, &Interaction, &Parent), Changed<Interaction>>,
    query_cmui_buttons: Query<&CMUIButton>,
) {
    for (entity, interaction, parent) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                state.selected = Some(entity);

                if let Ok(cmui_button) = query_cmui_buttons.get(**parent) {
                    event.send(ButtonPressEvent(cmui_button.key.clone()));
                }
            }
            Interaction::Hovered => {
                state.selected = Some(entity);
            }
            Interaction::None => {
                // state.selected = None;
            }
        }
    }
}

pub fn button_animation_system(
    state: Res<ButtonState>,
    mut query: Query<(Entity, &mut UiColor)>,
) {
    for (entity, mut color) in query.iter_mut() {
        if Some(entity) == state.selected {
            *color = CMUIButton::COLOR_ACTIVE.into();
        } else {
            *color = CMUIButton::COLOR_NORMAL.into();
        }
    }
}

pub fn selected_button_changed(mut state: ResMut<ButtonState>) -> bool {
    let statement = state.selected != state.prev_selected;

    if statement {
        state.prev_selected = state.selected;
    }

    statement
}

pub fn keyboard_button_interaction_system(
    mut event: EventWriter<ButtonPressEvent>,
    mut state: ResMut<ButtonState>,
    keyboard: Res<Input<KeyCode>>,
    query: Query<(Entity, &SelectedByDefault, &Parent)>,
    query_cmui_buttons: Query<&CMUIButton>,
) {
    if keyboard.any_just_pressed(CMUIButton::ITERACTION_KEYS) {
        let mut iter = query.iter();
        let first_entity = iter.nth(0).unwrap();
        let last_entity = iter.last().unwrap_or(first_entity).0;
        let first_entity = first_entity.0;

        for (index, (entity, selected_by_default, _)) in query.iter().enumerate() {
            if keyboard.just_pressed(KeyCode::S) || keyboard.just_pressed(KeyCode::W) {
                let is_next = keyboard.just_pressed(KeyCode::S);
                let (next_index, overflow_button) = if is_next {
                    (index + 1, first_entity)
                } else {
                    (
                        if index > 0 { index - 1 } else { query.iter().size_hint().1.unwrap() - 1 },
                        last_entity
                    )
                };

                if Some(entity) == state.selected || (state.selected.is_none() && selected_by_default.0) {
                    if let Some((next_entity, _, _)) = query.iter().nth(next_index) {
                        state.selected = Some(next_entity);
                    } else {
                        state.selected = Some(overflow_button);
                    };

                    break;
                }
            }
        }

        if keyboard.just_pressed(KeyCode::Return) {
            let (_, _, default) = query.iter()
                .find(|(_, selected_by_default, _)| { selected_by_default.0 })
                .expect("No selected by default button! You must specify selected button.");

            let selected = state.selected
                .and_then(|entity| {
                    if let Ok(button) = query.get(entity) {
                        Some(button)
                    } else {
                        None
                    }
                })
                .and_then(|(_, _, selected)| Some(selected))
                .unwrap_or(default);

            if let Ok(cmui_button) = query_cmui_buttons.get(**selected) {
                event.send(ButtonPressEvent(cmui_button.key.clone()));
            }
        }
    }
}

pub fn button_sfx_system(
    mut events: EventReader<ButtonPressEvent>,
    audio: Res<AudioChannel<UiAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
) {
    for _ in events.iter() {
        if let Some(sfx) = audio_storage.0.get(&AudioCollection::Button) {
            audio.play(sfx.clone());
        }
    }
}

pub fn clear_button_state(mut state: ResMut<ButtonState>) {
    if state.selected.is_some() {
        *state = ButtonState::default();
    }
}
