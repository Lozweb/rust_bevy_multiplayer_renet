use bevy::prelude::{App, IntoScheduleConfigs, MessageReader, Plugin, SystemSet, Update};
use bevy_renet::client_connected;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeTransportError};
use bevy_renet::renet::RenetClient;

use crate::resource::{ClientLobby, CurrentClientId};
use crate::system::client_event::on_client_event;
use game_core::network::connection_config;
use game_core::transport::setup_client_transport;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetcodeClientPlugin);

        let client = RenetClient::new(connection_config());
        let transport = setup_client_transport("127.0.0.1", 5000);

        app.insert_resource(CurrentClientId(transport.client_id()));
        app.insert_resource(client);
        app.insert_resource(transport);
        app.insert_resource(ClientLobby::default());

        app.add_systems(Update, panic_on_error_system);

        app.add_systems(Update, on_client_event.in_set(Connected));
        app.configure_sets(Update, Connected.run_if(client_connected));
    }
}

#[allow(clippy::never_loop)]
fn panic_on_error_system(mut renet_error: MessageReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}
