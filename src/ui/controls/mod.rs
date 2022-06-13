use bevy::prelude::*;
use iyes_loopless::prelude::*;

use self::button::{Button, button_animation_system, keyboard_button_interaction_system, handle_interaction_system, selected_button_changed, ButtonState, button_sfx_system, any_button_exist};

pub use self::button::ButtonPressEvent;

mod button;

#[derive(Default)]
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Controls>()
            .init_resource::<ButtonState>()
            .add_event::<ButtonPressEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_if(any_button_exist)
                    .with_system(button_animation_system.run_if(selected_button_changed))
                    .with_system(handle_interaction_system)
                    .with_system(keyboard_button_interaction_system)
                    .with_system(button_sfx_system)
                    .into()
            );
    }
}

pub struct Controls {
    pub font: Handle<Font>,
    pub header_font: Handle<Font>,
}

impl Controls {
    pub fn button(&self, text: &str) -> Button {
        Button { controls: self, id: None, text: text.to_string(), is_selected_by_default: false }
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

pub trait SpawnControl<'w, 's, 'a, T> {
    fn spawn_control(&mut self, control: T) -> T;
}
