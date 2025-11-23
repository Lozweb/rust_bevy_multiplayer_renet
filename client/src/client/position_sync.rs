use crate::game::player::{AimDirection, Player};
use crate::resource::{ClientLobby, CurrentClientId};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::server::ServerMessages;

/// Système qui reçoit et applique les mises à jour de position du serveur.
///
/// Ignore les updates du joueur local (déjà contrôlé directement).
/// Utilise le canal `NetworkedEntities` (unreliable) pour réduire la latence.
pub fn receive_position_updates(
    mut client: ResMut<RenetClient>,
    lobby: Res<ClientLobby>,
    mut players: Query<(&mut Transform, &mut AimDirection), With<Player>>,
    current_client_id: Option<Res<CurrentClientId>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        match ServerMessages::from_bytes(&message) {
            ServerMessages::PlayerPositionUpdate {
                client_id,
                position,
                velocity: _velocity,
                aim_direction,
            } => {
                if let Some(current_id) = &current_client_id {
                    if current_id.0 == client_id {
                        continue;
                    }
                }

                if let Some(player_entities) = lobby.get_player_entities(&client_id) {
                    if let Ok((mut transform, mut aim_dir)) =
                        players.get_mut(player_entities.client_entity)
                    {
                        transform.translation = position;
                        aim_dir.0 = aim_direction;

                        trace!(
                            "Updated position for player {}: {:?}, aim: {}",
                            client_id, position, aim_direction
                        );
                    } else {
                        warn!(
                            "Received position update for player {} but entity not found in query",
                            client_id
                        );
                    }
                } else {
                    trace!(
                        "Received position update for unknown player {}, ignoring",
                        client_id
                    );
                }
            }
            ServerMessages::PlayerCreate { .. }
            | ServerMessages::PlayerRemove { .. }
            | ServerMessages::ErrorMessage { .. } => {}
        }
    }
}
