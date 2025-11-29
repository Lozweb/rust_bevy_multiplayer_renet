use crate::client::handler::{
    enemy_death, enemy_spawned, player_create, player_remove, projectile_collision,
    projectile_spawned,
};
use crate::game::level::Level;
use crate::resource::{ClientLobby, CurrentClientId};
use bevy::log::error;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::server::ServerMessages;

pub fn on_client_event(
    current_client_id: Option<Res<CurrentClientId>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut commands: Commands,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
    level_query: Query<Entity, With<Level>>,
) {
    let Some(_current_client_id) = current_client_id else {
        return;
    };

    while let Some(event) = client.receive_message(ServerChannel::ReliableState) {
        match ServerMessages::from_bytes(&event) {
            ServerMessages::PlayerCreate {
                server_entity,
                client_id,
                position,
            } => player_create(
                client_id,
                server_entity,
                position,
                &mut lobby,
                &mut commands,
                &mut meshes,
                &mut materials,
                &level_query,
            ),
            ServerMessages::PlayerRemove { client_id } => {
                player_remove(client_id, &mut lobby, &mut commands)
            }
            ServerMessages::EnemySpawned {
                server_entity,
                enemy_type,
                position,
            } => enemy_spawned(
                server_entity,
                enemy_type,
                position,
                &mut lobby,
                &mut commands,
                &mut meshes,
                &mut materials,
            ),

            ServerMessages::EnemyDeath { server_entity } => {
                enemy_death(server_entity, &mut lobby, &mut commands)
            }

            ServerMessages::ProjectileSpawned {
                server_entity,
                damage,
                position,
                direction,
            } => projectile_spawned(
                server_entity,
                damage,
                position,
                direction,
                &mut lobby,
                &mut commands,
                &mut meshes,
                &mut materials,
            ),

            ServerMessages::ProjectileCollision { server_entity } => {
                projectile_collision(server_entity, &mut lobby, &mut commands)
            }

            ServerMessages::ErrorMessage { reason } => {
                error!("{}", reason);
            }
            _ => {}
        }
    }
}
