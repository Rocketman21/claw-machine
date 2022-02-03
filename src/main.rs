use assets::AssetLoaderPlugins;
use bevy::{prelude::*, DefaultPlugins};
use camera::CameraPlugin;
use movement::MovementPlugin;
use ui::UIPlugins;

mod camera;
mod assets;
mod movement;
mod ui;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugins)
        .add_plugins(UIPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(MovementPlugin)
        .add_state(GameState::Play)
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Play,
    Pause,
}
