use bevy::prelude::*;
use bevy_rapier3d::prelude::{ImpulseJoint, FixedJointBuilder};

#[derive(Default)]
pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(glue_system)
            .add_system_to_stage(CoreStage::PostUpdate, unglue_system);
    }
}

#[derive(Component)]
pub struct Glue(pub Entity);

fn glue_system(
    glue_query: Query<(Entity, &Glue), Without<ImpulseJoint>>,
    transform_query: Query<&Transform>,
    mut commands: Commands,
) {
    for (entity, glue) in glue_query.iter() {
        if let Ok(transform) = transform_query.get(glue.0) {
            let joint = ImpulseJoint::new(glue.0, FixedJointBuilder::new().local_basis1(transform.rotation));
            commands.entity(entity).insert(joint);
            println!("Приклеиваю {:?}", glue.0);
        }
    }
}

fn unglue_system(
    removed_glue: RemovedComponents<Glue>,
    mut commands: Commands,
) {
    for entity in removed_glue.iter() {
        println!("Deleting {:?}", entity);
        commands.entity(entity).remove::<ImpulseJoint>();
    }
}