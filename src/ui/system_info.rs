use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

#[derive(Default)]
pub struct SystemInfoPlugin;

#[derive(Component)]
struct Fps;

impl Plugin for SystemInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_system)
            .add_system(text_update_system)
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "FPS:",
                TextStyle {
                    font: asset_server.load("fonts/Coda-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
                Default::default()
            ),
            ..Default::default()
        })
        .insert(Fps);
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Fps>>) {
    for mut text in query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                            text.sections[0].value = format!("FPS: {:.0}", average);
                    }
            }
    }
}
