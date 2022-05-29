use bevy::{prelude::*, ecs::system::EntityCommands};

use super::{Controls, SpawnControl};

pub struct Button<'a> {
    pub controls: &'a Controls,
    pub text: String
}

impl<'a> Button<'a> {
    const COLOR_NORMAL: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    const COLOR_ACTIVE: Color = Color::rgb(
        114.0 / 255.0,
        0.0,
        163.0 / 255.0
    );
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
                    color: Button::COLOR_NORMAL.into(),
                    ..default()
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

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<bevy::prelude::Button>),
    >
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Button::COLOR_ACTIVE.into();
            }
            Interaction::Hovered => {
                *color = Button::COLOR_ACTIVE.into();
            }
            Interaction::None => {
                *color = Button::COLOR_NORMAL.into();
            }
        }
    }
}