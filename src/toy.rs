#[derive(Default)]
pub struct ToyPlugin;

impl Plugin for ToyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(toy_speed_control_system);
    }
}