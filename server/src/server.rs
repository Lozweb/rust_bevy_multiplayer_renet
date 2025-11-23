use crate::resource::server_lobby::ServerLobby;
use crate::resource::ServerConfig;
use crate::system::server_event::on_server_event;
use bevy::app::{App, Startup, Update};
use bevy::prelude::Res;
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::renet::RenetServer;
use game_core::network::connection_config;
use game_core::transport::setup_server_transport;
use tracing::info;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(connection_config());
    let transport = setup_server_transport("127.0.0.1", 5000);

    app.insert_resource(server);
    app.insert_resource(transport);
    app.insert_resource(ServerLobby::default());

    app.add_systems(Startup, setup_server);
    app.add_systems(Update, on_server_event);
}

fn setup_server(config: Res<ServerConfig>) {
    if config.headless {
        info!("Démarrage en mode headless sur le port {}", config.port);
    } else {
        info!(
            "Démarrage du serveur avec interface graphique sur le port {}",
            config.port
        );
    }
}
