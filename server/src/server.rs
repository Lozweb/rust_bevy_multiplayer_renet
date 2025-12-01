use crate::resource::server_lobby::ServerLobby;
use crate::{game, network};
use avian2d::PhysicsPlugins;
use bevy::app::App;
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::renet::RenetServer;
use game_core::enemy::EnemiesPositionTimer;
use game_core::network::connection_config;
use game_core::player::PlayersPositionTimer;
use game_core::transport::setup_server_transport;

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

    app.add_plugins((game::plugin, network::plugin));
}
