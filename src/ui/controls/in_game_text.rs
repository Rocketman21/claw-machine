use bevy::prelude::*;

use crate::constants::PURPLE_COLOR;

use super::{Controls, SpawnedControl};

#[derive(Component)]
pub struct InGameText(pub String);

pub fn in_game_text_spawner_system(
    controls: Res<Controls>,
    texts: Query<(Entity, &InGameText), Without<Text>>,
    mut commands: Commands
) {
    for (entity, text) in texts.iter() {
        commands.entity(entity)
            .remove::<InGameText>()
            .insert(SpawnedControl::<InGameText>::new())
            .insert_bundle(TextBundle {
                style: Style {
                    margin: Rect { top: Val::Percent(10.0), left: Val::Percent(20.0), ..default() },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for index in 0..=1 {
                    parent.spawn()
                        .insert(InGameText(text.0.clone()))
                        .insert_bundle(TextBundle {
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
                                text.0.clone(),
                                TextStyle {
                                    font: controls.header_font.clone(),
                                    font_size: 150.0,
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
    }
}
