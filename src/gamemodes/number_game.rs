use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;

use crate::{
    GameState,
    ui::controls::in_game_text::InGameText,
    claw::{
        ClawReturnedToBaseEvent,
        ClawController,
        ClawControllerState,
        ToyCatchEvent,
        ReleaseClawEvent
    },
    game_results::GameResults,
    assets::audio::{
        BackgroundAudioChannel,
        stop_background_audio_system,
        AudioHandleStorage,
        AudioCollection
    },
};

use super::gameplay::Gamemode;

#[derive(Default)]
pub struct NumberGamePlugin;

impl Plugin for NumberGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(Gamemode::NumberGame, setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(Gamemode::NumberGame)
                    .with_system(update_system)
                    .with_system(handle_claw_return_system.run_on_event::<ClawReturnedToBaseEvent>())
                    .with_system(increment_toys_system.run_on_event::<ToyCatchEvent>())
                    .into()
            )
            .add_exit_system(Gamemode::NumberGame, stop_background_audio_system)
            .add_exit_system(Gamemode::NumberGame, exit_system);
    }
}

#[derive(Component, Clone)]
pub struct NumberGameProgress {
    timer: Timer,
    heartbeat_played: bool,
    pub toys_caught: u8
}

impl NumberGameProgress {
    const TIME_TO_CATCH: f32 = 7.0;
}

impl Default for NumberGameProgress {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(NumberGameProgress::TIME_TO_CATCH, false),
            heartbeat_played: false,
            toys_caught: 0
        }
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn()
        .insert(NumberGameProgress::default())
        .insert(InGameText(String::new()));
}

fn update_system(
    time: Res<Time>,
    audio_background: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    mut progress_query: Query<&mut NumberGameProgress>,
    mut text_query: Query<&mut Text, With<InGameText>>,
    mut events: EventWriter<ReleaseClawEvent>,
) {
    if let Ok(mut progress) = progress_query.get_single_mut() {
        if progress.timer.tick(time.delta()).just_finished() {
            events.send(ReleaseClawEvent);
        }

        let remain = (
            NumberGameProgress::TIME_TO_CATCH - progress.timer.elapsed_secs()
        ).floor();

        if let Some(heartbeat) = audio_storage.0.get(&AudioCollection::Heartbeat) {
            if remain <= 5.0 && !progress.heartbeat_played {
                audio_background.set_volume(1.5);
                audio_background.play(heartbeat.clone());
                progress.heartbeat_played = true
            }
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = if remain > 0.0 {
                format!("{:.0}", remain)
            } else {
                String::new()
            };
        }
    }
}

fn increment_toys_system(
    mut query: Query<&mut NumberGameProgress>,
) {
    if let Ok(mut progress) = query.get_single_mut() {
        progress.toys_caught += 1;
    }
}

fn handle_claw_return_system(
    audio_background: Res<AudioChannel<BackgroundAudioChannel>>,
    mut claw_controller_query: Query<&mut ClawController>,
    progress_query: Query<&NumberGameProgress>,
    mut commands: Commands
) {
    if let (Ok(mut claw_controller), Ok(progress)) = (
        claw_controller_query.get_single_mut(),
        progress_query.get_single()
    ) {
        if progress.timer.finished() {
            audio_background.stop();
            commands.insert_resource(NextState(GameState::GameResults));
        } else {
            claw_controller.0 = ClawControllerState::Manual;
        }
    }
}

fn exit_system(
    query: Query<(Entity, &NumberGameProgress)>, 
    mut commands: Commands
) {
    let (entity, progress) = query.get_single().expect("number_game - exit_system");

    commands.spawn().insert(GameResults::NumberGame(progress.clone()));
    commands.entity(entity).despawn_recursive();
}