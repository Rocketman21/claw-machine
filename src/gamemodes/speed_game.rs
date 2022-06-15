use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate:: {
    gamemodes::gameplay::Gamemode,
    helpers::{despawn_with, event_received}, claw::{ReleaseClawEvent, ToyCatchEvent}, game_results::GameResults,
};

#[derive(Default)]
pub struct SpeedGamePlugin;

impl Plugin for SpeedGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(Gamemode::SpeedGame, setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(Gamemode::SpeedGame)
                    .with_system(register_toy_catch.run_if(event_received::<ToyCatchEvent>))
                    .with_system(speed_game_system)
                    .into()
            )
            .add_exit_system(Gamemode::SpeedGame, exit_system);
    }
}

#[derive(Component, Clone)]
pub struct SpeedGameProgress {
    pub timer: Timer,
    pub toy_caught: bool
}

impl Default for SpeedGameProgress {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(20.0, false), toy_caught: false }
    }
}

fn setup_system(
    mut commands: Commands,
) {
    commands.spawn().insert(SpeedGameProgress::default());
}

fn speed_game_system(
    time: Res<Time>,
    mut query: Query<&mut SpeedGameProgress>,
    mut events: EventWriter<ReleaseClawEvent>
) {
    if let Ok(mut progress) = query.get_single_mut() {
        println!("Tick, {}, caught: {}", progress.timer.elapsed_secs(), progress.toy_caught);
        if progress.timer.tick(time.delta()).just_finished() {
            events.send(ReleaseClawEvent);
        }
    }
}

fn register_toy_catch(mut query: Query<&mut SpeedGameProgress>) {
    println!("ToyCaughtEvent received");
    let mut progress = query.get_single_mut().expect("register_toy_catch");
    progress.toy_caught = true;
}

fn exit_system(
    query: Query<(Entity, &SpeedGameProgress)>, 
    mut commands: Commands
) {
    let (entity, progress) = query.get_single().expect("speed_game - exit_system");
    println!("Exit speed game");
    commands.spawn().insert(GameResults::SpeedGame(progress.clone()));
    commands.entity(entity).despawn_recursive();
}
