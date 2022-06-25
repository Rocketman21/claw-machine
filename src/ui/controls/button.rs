use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

use crate::{assets::audio::{UiAudioChannel, AudioHandleStorage, AudioCollection}, constants::PURPLE_COLOR};

use super::{Controls, SpawnControl};

pub struct Button<'a> {
    pub controls: &'a Controls,
    pub id: Option<Entity>,
    pub text: String,
    pub is_selected_by_default: bool
}

impl<'a> Button<'a> {
    const COLOR_NORMAL: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    const COLOR_ACTIVE: Color = PURPLE_COLOR;
    const ITERACTION_KEYS: [KeyCode; 3] = [
        KeyCode::W,
        KeyCode::S,
        KeyCode::Return
    ];

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
pub struct ButtonComponent;
#[derive(Component)]
pub struct SelectedByDefault(bool);

pub struct ButtonPressEvent(pub Entity);

impl<'w, 's, 'a> SpawnControl<'w, 's, 'a, Button<'a>> for ChildBuilder<'w, 's, '_> {
    fn spawn_control(&mut self, mut control: Button<'a>) -> Button<'a> {
        let mut entity_commands = self.spawn();

        entity_commands.insert_bundle(TextBundle::default());
        entity_commands.with_children(|button| {
            let id = button.spawn()
                .insert(ButtonComponent)
                .insert_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(280.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: Rect { top: Val::Px(10.0), ..default() },
                        ..default()
                    },
                    color: if control.is_selected_by_default {
                        Button::COLOR_ACTIVE.into()
                    } else {
                        Button::COLOR_NORMAL.into()
                    },
                    ..default()
                })
                .insert(SelectedByDefault(control.is_selected_by_default))
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
                                control.text.clone(),
                                TextStyle {
                                    font: control.controls.font.clone(),
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
                })
                .id();

                control.id = Some(id);
            });

        control
    }
}

pub fn any_button_exist(query: Query<&ButtonComponent>) -> bool {
    !query.is_empty()
}

pub fn handle_interaction_system(
    mut event: EventWriter<ButtonPressEvent>,
    mut state: ResMut<ButtonState>,
    mut interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>
) {
    for (entity, interaction) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                state.selected = Some(entity);
                event.send(ButtonPressEvent(entity));
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
            *color = Button::COLOR_ACTIVE.into();
        } else {
            *color = Button::COLOR_NORMAL.into();
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
    query: Query<(Entity, &SelectedByDefault)>
) {
    if keyboard.any_just_pressed(Button::ITERACTION_KEYS) {
        let mut iter = query.iter();
        let first_entity = iter.nth(0).unwrap().0;
        let last_entity = iter.last().unwrap().0;

        for (index, (entity, selected_by_default)) in query.iter().enumerate() {
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

                if Some(entity) == state.selected || selected_by_default.0 {
                    if let Some((next_entity, _)) = query.iter().nth(next_index) {
                        state.selected = Some(next_entity);
                    } else {
                        state.selected = Some(overflow_button);
                    };

                    break;
                }
            }
        }

        if keyboard.just_pressed(KeyCode::Return) {
            let (default, _) = query.iter()
                .find(|(_, selected_by_default)| { selected_by_default.0 })
                .expect("No selected by default button! You must specify selected button.");

            event.send(ButtonPressEvent(state.selected.unwrap_or(default)));
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
