use bevy::{prelude::*, app::PluginGroupBuilder};

use self::{main_menu::MenuPlugin, system_info::SystemInfoPlugin, controls::ControlsPlugin};

pub mod controls;
mod main_menu;
mod system_info;

#[derive(Default)]
pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(UIMainPlugin)
            .add(ControlsPlugin)
            .add(MenuPlugin)
            .add(SystemInfoPlugin);
    }
}

#[derive(Default)]
pub struct UIMainPlugin;

impl Plugin for UIMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system);
    }
}

pub trait CursorControl {
    fn toggle_cursor(&mut self, is_enabled: bool);
}

impl CursorControl for Window {
    fn toggle_cursor(&mut self, is_enabled: bool) {
        self.set_cursor_visibility(!is_enabled);
        self.set_cursor_lock_mode(is_enabled);
    }
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(UiCameraBundle::default());

    windows.get_primary_mut().and_then(|window| Some(window.toggle_cursor(false)));
}