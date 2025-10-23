use bevy::prelude::{App, MessageReader, Plugin, Update};
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport, NetcodeTransportError,
};
use bevy_renet::renet::RenetClient;

use crate::resource::{ClientLobby, CurrentClientId, PlayerMapping};
use game_core::network::{connection_config, get_current_time, get_socket, PROTOCOL_ID};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetcodeClientPlugin);

        build_client_transport(app);

        app.insert_resource(PlayerMapping::default());
        app.insert_resource(ClientLobby::default());
    }
}

fn build_client_transport(app: &mut App) {
    let client = RenetClient::new(connection_config());

    let server_addr = "127.0.0.1:5000"
        .parse()
        .expect("Failed to parse server address");
    let socket_addr = "127.0.0.1:0"
        .parse()
        .expect("Failed to parse socket address");
    let socket = get_socket(socket_addr);

    let current_time = get_current_time();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));

    #[allow(clippy::never_loop)]
    fn panic_on_error_system(mut renet_error: MessageReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            panic!("{}", e);
        }
    }

    app.add_systems(Update, panic_on_error_system);
}
