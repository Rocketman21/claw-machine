use std::marker::PhantomData;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use self::{button::{button_animation_system, keyboard_button_interaction_system, handle_interaction_system, selected_button_changed, ButtonState, button_sfx_system, button_spawner_system, CMUIButton}, menu::menu_spawner_system};

pub use self::button::ButtonPressEvent;

pub mod button;
pub mod menu;

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
                    .run_if(control_type_exist::<CMUIButton>)
                    .with_system(button_animation_system.run_if(selected_button_changed))
                    .with_system(handle_interaction_system)
                    .with_system(keyboard_button_interaction_system)
                    .with_system(button_sfx_system)
                    .into()
            )
            .add_system(button_spawner_system)
            .add_system(menu_spawner_system);
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
 
#[derive(Component)]
pub struct SpawnedControl<T> {
    control_type: PhantomData<T>,
}

impl<T> SpawnedControl<T> {
    fn new() -> Self {
        Self { control_type: PhantomData }
    }
}

pub fn control_type_exist<T: Component>(query: Query<With<SpawnedControl<T>>>) -> bool {
    !query.is_empty()
}