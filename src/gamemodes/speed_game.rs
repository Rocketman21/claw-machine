use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate:: {
    gamemodes::gameplay::Gamemode,
    claw::{ReleaseClawEvent, ToyCatchEvent}, game_results::GameResults, constants::PURPLE_COLOR, ui::controls::Controls,
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
                    .with_system(register_toy_catch.run_on_event::<ToyCatchEvent>())
                    .with_system(pause_timer.run_on_event::<ReleaseClawEvent>())
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

impl SpeedGameProgress {
    const TIME_TO_CATCH: f32 = 20.0;
}

impl Default for SpeedGameProgress {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(SpeedGameProgress::TIME_TO_CATCH, false), toy_caught: false }
    }
}

#[derive(Component)]
struct ProgressText;

fn setup_system(
    controls: Res<Controls>,
    mut commands: Commands,
) {
    commands.spawn()
        .insert(SpeedGameProgress::default())
        .insert_bundle(TextBundle {
            style: Style {
                margin: Rect { top: Val::Percent(10.0), left: Val::Percent(20.0), ..default() },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for index in 0..=1 {
                parent.spawn()
                    .insert(ProgressText)
                    .insert_bundle(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px((-index * 4) as f32),
                                left: Val::Px((-index * 4) as f32),
                                ..default()
                            },
                            ..default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: controls.header_font.clone(),
                                font_size: 150.0,
                                color: if index == 0 {
                                    PURPLE_COLOR
                                } else {
                                    Color::ANTIQUE_WHITE
                                },
                            },
                            default()
                        ),
                        ..default()
                    });
            }
        });
}

fn speed_game_system(
    time: Res<Time>,
    mut query_progress: Query<&mut SpeedGameProgress>,
    mut query_text: Query<&mut Text, With<ProgressText>>,
    mut events: EventWriter<ReleaseClawEvent>
) {
    if let Ok(mut progress) = query_progress.get_single_mut() {
        if !progress.timer.paused() && progress.timer.tick(time.delta()).just_finished() {
            progress.timer.pause();
            events.send(ReleaseClawEvent);
        }

        for mut text in query_text.iter_mut() {
            let elapsed = progress.timer.elapsed_secs();

            text.sections[0].value = format!("{:.2}", elapsed);
        }
    }
}

fn register_toy_catch(mut query: Query<&mut SpeedGameProgress>) {
    let mut progress = query.get_single_mut().expect("register_toy_catch");
    progress.toy_caught = true;
}

fn pause_timer(mut query: Query<&mut SpeedGameProgress>) {
    if let Ok(mut progress) = query.get_single_mut() {
        progress.timer.pause();
    }
}

fn exit_system(
    query: Query<(Entity, &SpeedGameProgress)>, 
    mut commands: Commands
) {
    let (entity, progress) = query.get_single().expect("speed_game - exit_system");

    commands.spawn().insert(GameResults::SpeedGame(progress.clone()));
    commands.entity(entity).despawn_recursive();
}
