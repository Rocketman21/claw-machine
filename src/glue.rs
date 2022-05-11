use bevy::prelude::*;
use bevy_rapier3d::prelude::{ImpulseJoint, FixedJoint};

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
    glue_query: Query<(Entity, &Glue), Without<ImpulseJoint>>,
    mut commands: Commands,
) {
    for (entity, glue) in glue_query.iter() {
        let joint = ImpulseJoint::new(glue.0, FixedJoint::new());
        println!("Приклеиваю {:?} к {:?}", entity, glue.0);
        commands.entity(entity).insert(joint);
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