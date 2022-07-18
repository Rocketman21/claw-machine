use bevy::{prelude::*, app::PluginGroupBuilder};

use self::{speed_game::SpeedGamePlugin, gameplay::GameplayPlugin, number_game::NumberGamePlugin};

pub mod gameplay;
pub mod speed_game;
pub mod number_game;

#[derive(Default)]
pub struct GamemodePlugins;

impl PluginGroup for GamemodePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(GameplayPlugin)
            .add(SpeedGamePlugin)
            .add(NumberGamePlugin);
    }
}