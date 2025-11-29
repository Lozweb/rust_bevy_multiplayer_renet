use crate::resource::server_lobby::ServerLobby;
use crate::resource::ServerConfig;
use crate::system::collision::collision;
use crate::system::input_handler::*;
use crate::system::position_sync::*;
use crate::system::server_event::on_server_event;
use avian2d::PhysicsPlugins;
use bevy::app::{App, FixedUpdate, Startup, Update};
use bevy::prelude::{IntoScheduleConfigs, Res};
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::renet::RenetServer;
use game_core::level::{setup_level, spawn_initial_enemies};
use game_core::network::connection_config;
use game_core::transport::setup_server_transport;
use tracing::info;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);
    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(avian2d::prelude::Gravity::ZERO);

    let server = RenetServer::new(connection_config());
    let transport = setup_server_transport("127.0.0.1", 5000);

    app.insert_resource(server);
    app.insert_resource(transport);
    app.insert_resource(ServerLobby::default());
    app.insert_resource(PlayersPositionTimer::default());
    app.insert_resource(EnemiesPositionTimer::default());

    app.add_systems(Startup, (setup_server, setup_level, spawn_initial_enemies));
    app.add_systems(Update, on_server_event);
    app.add_systems(Update, process_client_inputs);
    app.add_systems(
        Update,
        interpolate_movement_intent.after(process_client_inputs),
    );
    app.add_systems(Update, apply_movement.after(interpolate_movement_intent));
    app.add_systems(Update, sync_players_position.after(apply_movement));
    app.add_systems(FixedUpdate, sync_enemies_positions);
    app.add_systems(FixedUpdate, collision);
}

fn setup_server(config: Res<ServerConfig>) {
    info!(
        "Serveur démarré en mode console sur le port {}",
        config.port
    );
}
