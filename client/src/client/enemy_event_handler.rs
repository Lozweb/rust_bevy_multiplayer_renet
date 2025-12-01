use crate::client::client_event::{enemy_death, enemy_spawned};
use crate::resource::ClientLobby;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::{ColorMaterial, Commands, ResMut};
use game_core::server::EnemyMessages;

pub fn enemy_message(
    messages: EnemyMessages,
    lobby: &mut ResMut<ClientLobby>,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) {
    match messages {
        EnemyMessages::EnemySpawned {
            server_entity,
            enemy_type,
            position,
        } => enemy_spawned(
            server_entity,
            enemy_type,
            position,
            lobby,
            commands,
            meshes,
            materials,
        ),
        EnemyMessages::EnemyDeath { server_entity } => enemy_death(server_entity, lobby, commands),
    }
}
