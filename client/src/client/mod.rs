pub(crate) mod event;
pub mod input;
pub mod position_sync;

use crate::resource::ClientLobby;
use bevy::prelude::*;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeTransportError};

#[derive(Resource, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(NetcodeClientPlugin);

    app.insert_resource(ClientLobby::default());

    app.add_systems(Update, panic_on_error_system);
}

#[allow(clippy::never_loop)]
fn panic_on_error_system(mut renet_error: MessageReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        panic!("{}", e);
    }
}
