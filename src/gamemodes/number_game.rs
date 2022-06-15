use bevy::prelude::*;

#[derive(Default)]
pub struct NumberGamePlugin;

impl Plugin for NumberGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .add_enter_system(GameState::InGame, setup_system)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(countdown_system)
                    .into()
            );
    }
}