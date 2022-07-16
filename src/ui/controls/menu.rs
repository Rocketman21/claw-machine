use bevy::prelude::*;

use crate::constants::PURPLE_COLOR;

use super::{Controls, SpawnedControl, button::CMUIButton};

#[derive(Component)]
pub struct CMUIMenu {
    pub title: &'static str,
    pub buttons: Vec<CMUIButton>,
}

pub fn menu_spawner_system(
    controls: Res<Controls>,
    menus: Query<(Entity, &CMUIMenu), Without<SpawnedControl<CMUIMenu>>>,
    mut commands: Commands
) {
    for (entity, component) in menus.iter() {
        commands.entity(entity)
            .insert(SpawnedControl::<CMUIMenu>::new())
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
                    component.buttons.iter().for_each(|button| {
                        parent.spawn()
                            .insert_bundle(TextBundle::default())
                            .insert(button.clone());
                    })
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
                                        top: Val::Px((-index * 4) as f32),
                                        left: Val::Px((-index * 4) as f32),
                                        ..default()
                                    },
                                    ..default()
                                },
                                text: Text::with_section(
                                    component.title,
                                    TextStyle {
                                        font: controls.header_font.clone(),
                                        font_size: 100.0,
                                        color: if index == 0 {
                                            PURPLE_COLOR
                                        } else {
                                            Color::ANTIQUE_WHITE
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
}