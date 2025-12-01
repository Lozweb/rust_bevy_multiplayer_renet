use crate::client::client_event::{projectile_cleanup, projectile_collision, projectile_spawned};
use crate::resource::ClientLobby;
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::{ColorMaterial, Commands, ResMut};
use game_core::server::ProjectileMessages;

pub fn projectile_message(
    message: ProjectileMessages,
    lobby: &mut ResMut<ClientLobby>,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) {
    match message {
        ProjectileMessages::ProjectileSpawned {
            server_entity,
            damage,
            position,
            direction,
        } => projectile_spawned(
            server_entity,
            damage,
            position,
            direction,
            lobby,
            commands,
            meshes,
            materials,
        ),
        ProjectileMessages::ProjectileCollision { server_entity } => {
            projectile_collision(server_entity, lobby, commands)
        }
        ProjectileMessages::ProjectileCleanup { server_entity } => {
            projectile_cleanup(server_entity, lobby, commands)
        }
    }
}
