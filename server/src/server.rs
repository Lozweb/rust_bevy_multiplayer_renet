use crate::resource::server_lobby::ServerLobby;
use crate::resource::ServerConfig;
use crate::system::input_handler::{apply_movement, process_client_inputs};
use crate::system::position_sync::{broadcast_player_positions, PositionSyncTimer};
use crate::system::server_event::on_server_event;
use avian2d::PhysicsPlugins;
use bevy::app::{App, Startup, Update};
use bevy::prelude::{IntoScheduleConfigs, Res};
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::renet::RenetServer;
use game_core::network::connection_config;
use game_core::transport::setup_server_transport;
use tracing::info;

/// Plugin principal du serveur.
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);
    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(avian2d::prelude::Gravity::ZERO);

    let server = RenetServer::new(connection_config());
    let transport = setup_server_transport("127.0.0.1", 5000);

    app.insert_resource(server);
    app.insert_resource(transport);
    app.insert_resource(ServerLobby::default());
    app.insert_resource(PositionSyncTimer::default());

    app.add_systems(Startup, setup_server);
    app.add_systems(Update, on_server_event);
    app.add_systems(Update, process_client_inputs);
    app.add_systems(Update, apply_movement.after(process_client_inputs));
    app.add_systems(Update, broadcast_player_positions.after(apply_movement));
}

/// Système de démarrage : affiche le mode de lancement du serveur.
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
