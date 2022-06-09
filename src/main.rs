use assets::AssetLoaderPlugins;
use bevy::{prelude::*, DefaultPlugins};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use claw::ClawPlugin;
use claw_machine::ClawMachinePlugin;
use controls::ControlsPlugin;
use glue::GluePlugin;
use iyes_loopless::prelude::*;
use movement::MovementPlugin;
use room::RoomPlugin;
use toy::ToyPlugin;
use ui::UIPlugins;

mod camera;
mod assets;
mod movement;
mod ui;
mod controls;
mod claw;
mod toy;
mod constants;
mod glue;
mod room;
mod claw_machine;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))

        .add_loopless_state(GameState::Loading)

        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugins(AssetLoaderPlugins)
        .add_plugins(UIPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(ClawPlugin)
        .add_plugin(ToyPlugin)
        .add_plugin(GluePlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(ClawMachinePlugin)

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    SpeedGame,
    NumberGame
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    InGame(GameMode),
}
