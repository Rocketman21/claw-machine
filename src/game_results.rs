use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{gamemodes::speed_game::SpeedGameProgress, GameState};

#[derive(Default)]
pub struct GameResultsPlugin;

impl Plugin for GameResultsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::ResultsMenu, setup_system);
            // .add_system_set(
            //     ConditionSet::new()
            //         .run_in_state(GameState::InGame)
            //         .with_system(countdown_system)
            //         .into()
            // )
            // .add_exit_system(GameState::InGame, despawn_with::<Countdown>);
            // .add_exit_system(GameState::InGame, exit_system);
    }
}

#[derive(Component)]
pub enum GameResults {
    SpeedGame(SpeedGameProgress)
}

fn setup_system(query: Query<&GameResults>) {
    let results = query.get_single().expect("game_results - setup_system");
    println!("Enter results");
    match results {
        GameResults::SpeedGame(progress) => {
            println!("Game results: Time: {}, Toy: {}", progress.timer.elapsed_secs(), progress.toy_caught);
        }
    }
}