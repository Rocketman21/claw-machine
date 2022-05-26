use bevy::prelude::*;

pub struct Button {
    pub component: ButtonBundle,
    pub children: TextBundle,
}

impl Button {
    const COLOR_NORMAL: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    const COLOR_ACTIVE: Color = Color::rgb(
        114.0 / 255.0,
        0.0,
        163.0 / 255.0
    );

    pub fn with_font(font: Handle<Font>, text: &str) -> Self {
        Self {
            component: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Button::COLOR_NORMAL.into(),
                ..default()
            },
            children: TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    default(),
                ),
                ..default()
            }
        }
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