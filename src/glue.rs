use bevy::prelude::*;
use bevy_rapier3d::prelude::{ImpulseJoint, FixedJointBuilder};

#[derive(Default)]
pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(glue_system)
            .add_system_to_stage(CoreStage::Last, unglue_system);
    }
}

#[derive(Component)]
pub struct Glue(pub Entity);

fn glue_system(
    glue_query: Query<(Entity, &Transform, &Glue), Without<ImpulseJoint>>,
    mut commands: Commands,
) {
    for (entity, transform, glue) in glue_query.iter() {
        let joint = ImpulseJoint::new(entity, FixedJointBuilder::new().local_basis1(transform.rotation));
        commands.entity(glue.0).insert(joint);
    }
}

fn unglue_system(
    removed_glue: RemovedComponents<Glue>,
    mut commands: Commands,
) {
    for entity in removed_glue.iter() {
        commands.entity(entity).remove::<ImpulseJoint>();
    }
}