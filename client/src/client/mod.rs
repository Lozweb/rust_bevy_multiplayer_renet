mod enemy_message;
mod handler;
pub(crate) mod message_routing;
pub mod player_input;
mod player_message;
pub mod position_sync;
mod projectil_message;

use crate::resource::ClientLobby;
use bevy::prelude::*;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeTransportError};
use bevy_renet::renet::RenetClient;

#[derive(Resource, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

#[derive(Event)]
pub struct DisconnectUser;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(NetcodeClientPlugin);

    app.insert_resource(ClientLobby::default());

    app.add_systems(Update, panic_on_error_system);

    app.add_observer(handle_disconnect_user);
}

#[allow(clippy::never_loop)]
fn panic_on_error_system(mut renet_error: MessageReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}

fn handle_disconnect_user(_trigger: On<DisconnectUser>, mut client: ResMut<RenetClient>) {
    client.disconnect();
    info!("Déconnexion du serveur demandée par l'utilisateur");
}
