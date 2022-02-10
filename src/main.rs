use assets::AssetLoaderPlugins;
use bevy::{prelude::*, DefaultPlugins};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use claw::ClawPlugin;
use controls::ControlsPlugin;
use movement::MovementPlugin;
use ui::UIPlugins;

mod camera;
mod assets;
mod movement;
mod ui;
mod controls;
mod claw;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugins(AssetLoaderPlugins)
        .add_plugins(UIPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(ClawPlugin)

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)

        .add_state(GameState::Play)
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Play,
    Pause,
}
