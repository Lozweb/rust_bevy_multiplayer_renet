use crate::plugin::game_plugin::Connected;
use crate::system::input::input_sync_system;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Update};
use bevy_renet::client_connected;
use game_core::player::{AimDirection, MouseWorldCoords, PlayerInput};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default());
        app.insert_resource(MouseWorldCoords::default());
        app.insert_resource(AimDirection::default());

        app.add_systems(Update, input_sync_system.in_set(Connected));
        app.configure_sets(Update, Connected.run_if(client_connected));
    }
}
