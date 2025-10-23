use crate::resource::{ClientLobby, CurrentClientId, PlayerMapping};
use bevy::log::error;
use bevy::prelude::{info, Assets, ColorMaterial, Commands, Mesh, Res, ResMut};
use bevy_renet::renet::RenetClient;
use game_core::client::PlayerEntities;
use game_core::network::deserialize_server_message;
use game_core::player::{spawn_player, ControlledPlayer};
use game_core::server::{ServerChannel, ServerMessages};

pub fn on_server_event(
    current_client_id: Res<CurrentClientId>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut player_mapping: ResMut<PlayerMapping>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    while let Some(event) = client.receive_message(ServerChannel::ServerMessages) {
        match deserialize_server_message(&event).0 {
            ServerMessages::PlayerCreate {
                client_id,
                entity,
                position,
            } => {
                info!("Player created: {client_id} at {position:?} with entity {entity}");
                let player = spawn_player(
                    &client_id,
                    position,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );

                if current_client_id.0 == client_id {
                    commands.entity(entity).insert(ControlledPlayer);
                }

                lobby.add_player(
                    &client_id,
                    PlayerEntities {
                        server_entity: entity,
                        client_entity: player,
                    },
                );
                player_mapping.add(entity, player);
            }
            ServerMessages::PlayerRemove { client_id } => {
                info!("Player removed: {client_id}");
                if let Some(PlayerEntities {
                    server_entity,
                    client_entity,
                }) = lobby.remove_player(&client_id)
                {
                    commands.entity(client_entity).despawn();
                    player_mapping.remove(&server_entity);
                }
            }
            ServerMessages::Error { message } => {
                error!("Server error message: {}", message);
            }
        }
    }
}
