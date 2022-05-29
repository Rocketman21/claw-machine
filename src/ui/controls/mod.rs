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

impl Controls {
    pub fn button(&self, text: &str) -> Button {
        Button { controls: self, text: text.to_string() }
    }
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

pub trait SpawnControl<'w, 's, T> {
    fn spawn_control(&mut self, control: T) -> EntityCommands<'w, 's, '_>;
}
