use crate::handler::player_event;
use crate::resource::server_lobby::ServerLobby;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};
use game_core::enemy::{Enemy, EnemyServerEntity};
use game_core::player::PlayerInfo;

pub fn server_event(
    players: Query<(Entity, &PlayerInfo, &Transform)>,
    enemies: Query<(&Transform, &Enemy, &EnemyServerEntity)>,
    mut server: ResMut<RenetServer>,
    mut lobby: ResMut<ServerLobby>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
    mut server_event_reader: MessageReader<ServerEvent>,
    mut commands: Commands,
) {
    for event in server_event_reader.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => player_event::client_connected(
                *client_id,
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut server,
                &mut lobby,
                players,
                enemies,
            ),
            ServerEvent::ClientDisconnected { client_id, .. } => player_event::client_disconnected(
                *client_id,
                &mut lobby,
                &mut commands,
                &mut server,
            ),
        }
    }
}
