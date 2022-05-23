use std::f32::consts::PI;

use bevy::{prelude::*, gltf::Gltf};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::gltf::{GltfHandleStorage, GltfCollection},
    constants::{COL_GROUP_ALL, COL_GROUP_CLAW, COL_GROUP_TOY_EJECTION_SHELV, COL_GROUP_EJECTED_TOY}, GameState
};

#[derive(Default)]
pub struct ToyPlugin;
#[derive(Component)]
pub struct Toy;
#[derive(Component)]
pub struct ToySensor;

impl Toy {
    const MAX_SPEED: f32 = 2.0;
}

impl Plugin for ToyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_exit_system(GameState::Loading, spawn_toys_system)
            .add_system(toy_speed_control_system);
    }
}

fn toy_speed_control_system(mut query: Query<&mut Velocity, With<Toy>>) {
    for mut velocity in query.iter_mut() {
        if velocity.linvel.abs().max_element() > Toy::MAX_SPEED {
            velocity.linvel = velocity.linvel.clamp_length_max(Toy::MAX_SPEED);
        }
    }
}

fn spawn_toys_system(
    assets: Res<Assets<Gltf>>,
    asset_storage: Res<GltfHandleStorage>,
    mut commands: Commands
) {
    if let Some(gltf) = assets.get(asset_storage.0.get(&GltfCollection::HighLander).unwrap()) {
        let size = (0.1, 0.40, 0.25); // true collision is (0.1, 0.44, 0.25)
        let copies = 15;
        let radius = 0.5;

        for index in 1..copies + 1 {
            let angle = 360.0 / index as f32 * 180.0 / PI;

            commands.spawn()
                .insert(Toy)
                .insert(RigidBody::Dynamic)
                .insert(Transform {
                    translation: Vec3::new(radius * f32::sin(angle), 2.5, radius * f32::cos(angle)),
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                })
                .insert(GlobalTransform::identity())
                .insert(Collider::round_cuboid(size.0, size.1, size.2, 0.02))
                .insert(CollisionGroups::new(
                    COL_GROUP_ALL,
                    COL_GROUP_ALL - COL_GROUP_CLAW - COL_GROUP_TOY_EJECTION_SHELV - COL_GROUP_EJECTED_TOY
                ))
                .insert(Velocity::default())
                .with_children(|parent| {
                    parent.spawn()
                        .insert_bundle((Transform::from_xyz(0.0, -size.1, 0.0), GlobalTransform::identity()))
                        .with_children(|parent| {
                            parent.spawn_scene(gltf.scenes[0].clone());
                        });
                    
                    parent.spawn()
                        .insert(ToySensor)
                        .insert(Collider::ball(0.2))
                        .insert(Sensor(true));

                });
        }
    }
}