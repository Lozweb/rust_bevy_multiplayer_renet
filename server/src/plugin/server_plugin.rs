use crate::resource::ServerLobby;
use crate::system::server_event::on_server_event;
use bevy::app::{App, Plugin, Update};
use bevy_renet::netcode::{
    NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig,
};
use bevy_renet::renet::RenetServer;
use game_core::network::{connection_config, get_current_time, get_socket, PROTOCOL_ID};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetcodeServerPlugin);

        build_server_transport(app);

        app.add_systems(Update, on_server_event);
    }
}

fn build_server_transport(app: &mut App) {
    let server = RenetServer::new(connection_config());
    let public_addr = "127.0.0.1:5000"
        .parse()
        .expect("Failed to parse public address");
    let socket = get_socket(public_addr);
    let current_time = get_current_time();

    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    app.insert_resource(server);
    app.insert_resource(transport);
    app.insert_resource(ServerLobby::default());
}
