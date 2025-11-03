use crate::resource::ServerLobby;
use crate::system::camera::spawn_camera;
use crate::system::server_event::on_server_event;
use bevy::app::{App, Plugin, Startup, Update};
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::renet::RenetServer;
use game_core::network::connection_config;
use game_core::transport::setup_server_transport;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetcodeServerPlugin);

        let server = RenetServer::new(connection_config());
        let transport = setup_server_transport("127.0.0.1", 5000);

        app.insert_resource(server);
        app.insert_resource(transport);
        app.insert_resource(ServerLobby::default());

        app.add_systems(Update, on_server_event);
        app.add_systems(Startup, spawn_camera);
    }
}
