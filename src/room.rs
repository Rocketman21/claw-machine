use std::f32::consts::PI;

use bevy::{prelude::*, gltf::Gltf};
use iyes_loopless::prelude::*;

use crate::{assets::gltf::{GltfHandleStorage, GltfCollection}, GameState};

#[derive(Default)]
pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_exit_system(GameState::Loading, spawn_room_system);
    }
}

fn spawn_room_system(
    assets: Res<Assets<Gltf>>,
    asset_storage: Res<GltfHandleStorage>,
    mut commands: Commands
) {
    if let Some(gltf) = assets.get(asset_storage.0.get(&GltfCollection::Room).unwrap()) {
        commands.spawn()
            .insert_bundle((
                Transform {
                    scale: [2.2, 2.2, 2.2].into(),
                    rotation: Quat::from_rotation_y(-180.0 * PI / 180.),
                    translation: [-2.2, 0., 5.4].into(),
                    ..Default::default()
                },
                GlobalTransform::identity())
            )
            .with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });

        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(2.0, 5.0, 4.0)),
            ..Default::default()
        });
    }
}