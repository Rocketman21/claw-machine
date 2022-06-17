use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use iyes_loopless::prelude::*;

use crate::{
    claw::{ClawController, ClawControllerState},
    GameState, assets::audio::{UiAudioChannel, AudioHandleStorage, AudioCollection, BackgroundAudioChannel},
    ui::controls::Controls, constants::PURPLE_COLOR, helpers::despawn_with,
};

#[derive(Default)]
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .add_loopless_state(Gamemode::None)
            .add_enter_system(GameState::InGame, setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(countdown_system)
                    .into()
            )
            .add_exit_system(GameState::InGame, despawn_with::<Countdown>)
            .add_exit_system(GameState::InGame, exit_system);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gamemode {
    None,
    SpeedGame,
    NumberGame
}

pub struct GameSettings {
    pub gamemode: Gamemode
}

impl Default for GameSettings {
    fn default() -> Self {
        Self { gamemode: Gamemode::None }
    }
}

#[derive(Component)]
struct Countdown(Timer);
#[derive(Component)]
struct CountdownText;

const GAMEPLAY_MUSIC: [AudioCollection; 3] = [
    AudioCollection::Gameplay1,
    AudioCollection::Gameplay2,
    AudioCollection::Gameplay3
];

fn setup_system(
    audio: Res<AudioChannel<UiAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    controls: Res<Controls>,
    mut commands: Commands,
) {
    if let Some(countdown) = audio_storage.0.get(&AudioCollection::Countdown) {
        let secs = 3.0;

        commands.spawn()
            .insert(Countdown(Timer::from_seconds(secs, false)))
            .insert_bundle(TextBundle {
                style: Style {
                    margin: Rect { top: Val::Percent(10.0), left: Val::Percent(50.0), ..default() },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for index in 0..=1 {
                    parent.spawn()
                        .insert(CountdownText)
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
                                secs.to_string(),
                                TextStyle {
                                    font: controls.header_font.clone(),
                                    font_size: 200.0,
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

        audio.play(countdown.clone());
    }
}

fn countdown_system(
    time: Res<Time>,
    audio: Res<AudioChannel<BackgroundAudioChannel>>,
    audio_storage: Res<AudioHandleStorage>,
    settings: Res<GameSettings>,
    mut query_countdown: Query<(Entity, &mut Countdown)>,
    mut query_text: Query<&mut Text, With<CountdownText>>,
    mut query_claw: Query<&mut ClawController>,
    mut commands: Commands,
) {
    if let Ok((entity, mut countdown)) = query_countdown.get_single_mut() {
        if countdown.0.tick(time.delta()).just_finished() {
            if let Ok(mut claw_controller) = query_claw.get_single_mut() {
                if let Some(music) = audio_storage.get_random(&GAMEPLAY_MUSIC) {
                    audio.play_looped(music.clone());
                }

                claw_controller.0 = ClawControllerState::Manual;

                commands.insert_resource(NextState(settings.gamemode));
                commands.entity(entity).despawn_recursive();
            }
        }

        for mut text in query_text.iter_mut() {
            text.sections[0].value = (3 - countdown.0.elapsed().as_secs()).to_string();
        }
    }
}

fn exit_system(
    mut commands: Commands
) {
    commands.insert_resource(NextState(Gamemode::None));
}