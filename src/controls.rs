use bevy::{prelude::*, input::{keyboard::KeyboardInput, ElementState}, utils::HashMap};

/// Implicitly maps some scan codes to key codes to make
/// [`Input<KeyCode>`](Input) resource work fine
/// with any keyboard layout just like you expect it to be
#[derive(Default)]
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KeyMap>()
            .add_startup_system(register_keymap)
            .add_system(controls_system);
    }
}

#[derive(Default)]
struct KeyMap(HashMap<u32, KeyCode>);

fn register_keymap(mut key_map: ResMut<KeyMap>) {
    key_map.0.insert(17, KeyCode::W);
    key_map.0.insert(30, KeyCode::A);
    key_map.0.insert(31, KeyCode::S);
    key_map.0.insert(32, KeyCode::D);
}

fn controls_system(
    key_map: Res<KeyMap>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut keyboard_events: EventReader<KeyboardInput>
) {
    for event in keyboard_events.iter() {
        match event.state {
            ElementState::Pressed => key_map
                .0
                .get(&event.scan_code)
                .and_then(|key_code| Some(keyboard_input.press(*key_code))),
            ElementState::Released => key_map
                .0
                .get(&event.scan_code)
                .and_then(|key_code| Some(keyboard_input.release(*key_code))),
        };
    }
}