use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::gameplay::Gamemode;

#[derive(Default)]
pub struct NumberGamePlugin;

impl Plugin for NumberGamePlugin {
    fn build(&self, app: &mut App) {
        // app
            // .add_enter_system(Gamemode::NumberGame, setup_system)
            // .add_system_set(
            //     ConditionSet::new()
            //         .run_in_state(Gamemode::NumberGame)
            //         .with_system(countdown_system)
            //         .into()
            // );
    }
}