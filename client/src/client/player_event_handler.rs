use crate::client::client_event::{player_create, player_remove};
use crate::game::level::Level;
use crate::resource::ClientLobby;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::{ColorMaterial, Commands, Entity, Query, ResMut, With};
use game_core::server::PlayerMessages;

pub fn player_message(
    message: PlayerMessages,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
    level_query: &Query<Entity, With<Level>>,
) {
    match message {
        PlayerMessages::PlayerCreate {
            server_entity,
            client_id,
            position,
        } => player_create(
            client_id,
            server_entity,
            position,
            lobby,
            commands,
            meshes,
            materials,
            level_query,
        ),
        PlayerMessages::PlayerRemove { client_id } => player_remove(client_id, lobby, commands),
    }
}
