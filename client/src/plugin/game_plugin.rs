use crate::system::client_event::on_server_event;
use bevy::app::Update;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, SystemSet};
use bevy_renet::client_connected;
use game_core::event::game_event::GameEvent;
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameEvent>();
        app.add_systems(Update, on_server_event);

        app.add_systems(Update, on_server_event.in_set(Connected));
        app.configure_sets(Update, Connected.run_if(client_connected));
    }
}
