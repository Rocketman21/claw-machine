use bevy::{prelude::*, ecs::system::EntityCommands};

use self::button::{Button, button_system};

mod button;

#[derive(Default)]
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Controls>()
            .add_system(button_system);
    }
}

pub struct Controls {
    pub font: Handle<Font>,
    pub header_font: Handle<Font>,
}

impl FromWorld for Controls {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        Self {
            font: asset_server.load("fonts/RussoOne-Regular.ttf"),
            header_font: asset_server.load("fonts/Blaka-Regular.ttf")
        }
    }
}

impl Controls {
    pub fn button(&self, text: &str) -> Button {
        Button::with_font(self.font.clone(), text)
    }
}

pub trait InsertControls {
    fn insert_button(&mut self, button: Button) -> &mut Self;
}

impl<'w, 's, 'a> InsertControls for EntityCommands<'w, 's, 'a> {
    fn insert_button(&mut self, button: Button) -> &mut Self {
        self
            .insert_bundle(button.component)
            .with_children(|parent| { parent.spawn_bundle(button.children); })
    }
}
