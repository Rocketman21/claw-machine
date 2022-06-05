use bevy::{prelude::*, ecs::system::EntityCommands};

use super::{Controls, SpawnControl};

pub struct Button<'a> {
    pub controls: &'a Controls,
    pub text: String,
    pub is_selected_by_default: bool
}

impl<'a> Button<'a> {
    const COLOR_NORMAL: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    const COLOR_ACTIVE: Color = Color::rgb(
        114.0 / 255.0,
        0.0,
        163.0 / 255.0
    );
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

#[derive(Component)]
pub enum ButtonState {
    None,
    Selected,
    Pressed
}

impl<'w, 's> SpawnControl<'w, 's, Button<'_>> for ChildBuilder<'w, 's, '_> {
    fn spawn_control(&mut self, control: Button) -> EntityCommands<'w, 's, '_> {
        let mut entity_commands = self.spawn();

        entity_commands.insert_bundle(TextBundle::default());
        entity_commands.with_children(|button| {
            button
                .spawn_bundle(ButtonBundle {
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
                .insert(if control.is_selected_by_default {
                    ButtonState::Selected
                } else {
                    ButtonState::None
                })
                .with_children(|parent| {
                    for index in 0..=1 {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: Rect {
                                    top: Val::Px(14.0 - index as f32),
                                    left: Val::Px(20.0 - index as f32),
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
                                        Color::GRAY
                                    } else {
                                        Color::WHITE
                                    },
                                },
                                default(),
                            ),
                            ..default()
                        });
                    }
                });
            });

        entity_commands
    }
}

pub fn handle_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut ButtonState),
        Changed<Interaction>,
    >
) {
    for (interaction, mut button_state) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *button_state = ButtonState::Pressed;
            }
            Interaction::Hovered => {
                *button_state = ButtonState::Selected;
            }
            Interaction::None => {
                *button_state = ButtonState::None;
            }
        }
    }
}

pub fn button_animation_system(
    mut query: Query<(Entity, &mut ButtonState, &mut UiColor)>,
) {
    let mut selected: Option<Entity> = None;

    println!("Оно запустилось");

    for (entity, state, mut color) in query.iter_mut() {
        match *state {
            ButtonState::Selected => {
                *color = Button::COLOR_ACTIVE.into();
                selected = Some(entity);
            },
            ButtonState::None => {
                *color = Button::COLOR_NORMAL.into();
            },
            _ => {}
        }
    }

    if let Some(just_selected_button) = selected {
        query.iter_mut().for_each(|(entity, mut state, _)| {
            if let ButtonState::Selected = *state {
                if entity != just_selected_button {
                    *state = ButtonState::None;
                }
            }
        })
    }
}

pub fn some_button_changed(query: Query<Entity, Changed<ButtonState>>) -> bool {
    println!("asda {:?}", query.iter().size_hint().1);
    query.iter().size_hint().1.unwrap_or(0) > 0
}

pub fn keyboard_button_interaction_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut ButtonState)>
) {
    if keyboard.any_just_pressed(Button::ITERACTION_KEYS) {
        let mut iter = query.iter();
        let first_entity = iter.nth(0).unwrap().0;
        let last_entity = iter.last().unwrap().0;
        let mut selected_button = first_entity;

        for (index, (_, state)) in query.iter().enumerate() {
            if keyboard.just_pressed(KeyCode::S) || keyboard.just_pressed(KeyCode::W) {
                let is_next = keyboard.just_pressed(KeyCode::S);
                let (next_index, overflow_button) = if is_next {
                    (index + 1, first_entity)
                } else {
                    (index - 1, last_entity)
                };

                if let ButtonState::Selected = state {
                    if let Some((next_entity, _)) = query.iter().nth(next_index) {
                        selected_button = next_entity;
                    } else {
                        selected_button = overflow_button;
                    };

                    break;
                }
            }
        }

        *query.get_mut(selected_button).unwrap().1 = ButtonState::Selected;
    }
}