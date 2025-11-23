use crate::game::level::Level;
use crate::game::player::player;
use crate::resource::{ClientLobby, CurrentClientId, PlayerEntities};
use bevy::log::error;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::player::ControlledPlayer;
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
    // Si le client n'est pas connecté, ne rien faire
    let Some(current_client_id) = current_client_id else {
        return;
    };

    while let Some(event) = client.receive_message(ServerChannel::ServerMessages) {
        match ServerMessages::from_bytes(&event) {
            ServerMessages::PlayerCreate {
                client_id,
                entity,
                position,
            } => {
                let player_exists_by_id = lobby.get_player_entities(&client_id).is_some();
                let player_exists_by_entity = lobby.get_player_by_server_entity(&entity).is_some();

                if !player_exists_by_id
                    && !player_exists_by_entity
                    && let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut())
                    && let Ok(level_entity) = level_query.single()
                {
                    let player_bundle = player(client_id, position, 400., materials, meshes);
                    let is_local_player = current_client_id.0 == client_id;

                    // Spawn le joueur comme enfant du level
                    let mut player_entity_id = None;
                    commands.entity(level_entity).with_children(|parent| {
                        let mut entity_commands = parent.spawn(player_bundle);

                        // Ajouter ControlledPlayer uniquement si c'est le joueur local
                        if is_local_player {
                            entity_commands.insert(ControlledPlayer);
                        }

                        player_entity_id = Some(entity_commands.id());
                    });

                    // Enregistrer le joueur dans le lobby
                    if let Some(player_id) = player_entity_id {
                        lobby.add_player(
                            &client_id,
                            PlayerEntities {
                                server_entity: entity,
                                client_entity: player_id,
                            },
                        );
                        info!(
                            "Player created: {client_id} at {position:?} with entity {entity} (local: {is_local_player})"
                        );
                    }
                } else if player_exists_by_id || player_exists_by_entity {
                    info!(
                        "Player creation ignored for existing player: {client_id} (already exists)"
                    );
                } else {
                    error!("Player creation failed: missing resources or level entity");
                }
            }
            ServerMessages::PlayerRemove { client_id } => {
                info!("Player removed: {client_id}");
                if let Some(PlayerEntities {
                    server_entity: _server_entity,
                    client_entity,
                }) = lobby.remove_player(&client_id)
                {
                    commands.entity(client_entity).despawn();
                }
            }
            ServerMessages::ErrorMessage { reason } => {
                error!("{}", reason);
            }
        }
    }
}

pub fn server_network_sync(mut client: ResMut<RenetClient>) {
    while let Some(event) = client.receive_message(ServerChannel::NetworkedEntities) {
        ServerMessages::from_bytes(&event);
        { /* Ignore other messages */ }
    }
}
