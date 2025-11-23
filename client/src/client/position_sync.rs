use crate::game::player::{AimDirection, Player};
use crate::resource::{ClientLobby, CurrentClientId};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::server::ServerMessages;

/// Système qui reçoit et applique les mises à jour de position du serveur.
///
/// Pour chaque message `PlayerPositionUpdate` reçu :
/// - Ignore le message si c'est pour le joueur local (déjà contrôlé directement)
/// - Trouve l'entité du joueur via le `ClientLobby`
/// - Applique la position et aim_direction reçues du serveur
///
/// Le canal `NetworkedEntities` (unreliable) est utilisé car :
/// - Les updates arrivent fréquemment (20 Hz)
/// - La perte d'un paquet n'est pas critique (le prochain corrigera)
/// - Réduit la latence
pub fn receive_position_updates(
    mut client: ResMut<RenetClient>,
    lobby: Res<ClientLobby>,
    mut players: Query<(&mut Transform, &mut AimDirection), With<Player>>,
    current_client_id: Option<Res<CurrentClientId>>,
) {
    // Lire tous les messages sur le canal NetworkedEntities
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        match ServerMessages::from_bytes(&message) {
            ServerMessages::PlayerPositionUpdate {
                client_id,
                position,
                velocity: _velocity, // TODO: Utiliser pour l'interpolation future
                aim_direction,
            } => {
                // NE PAS appliquer au joueur local (on le contrôle déjà)
                if let Some(current_id) = &current_client_id {
                    if current_id.0 == client_id {
                        continue; // Skip le joueur local
                    }
                }

                // Trouver l'entité du joueur via le lobby
                if let Some(player_entities) = lobby.get_player_entities(&client_id) {
                    // Appliquer la position et aim_direction reçus
                    if let Ok((mut transform, mut aim_dir)) =
                        players.get_mut(player_entities.client_entity)
                    {
                        transform.translation = position;
                        aim_dir.0 = aim_direction;

                        trace!(
                            "Updated position for player {}: {:?}, aim: {}",
                            client_id, position, aim_direction
                        );

                        // TODO: Stocker velocity pour interpolation future
                    } else {
                        warn!(
                            "Received position update for player {} but entity not found in query",
                            client_id
                        );
                    }
                } else {
                    // Le joueur n'est pas encore dans le lobby (message arrivé avant PlayerCreate)
                    trace!(
                        "Received position update for unknown player {}, ignoring",
                        client_id
                    );
                }
            }
            // Ignorer les autres types de messages sur ce canal
            ServerMessages::PlayerCreate { .. }
            | ServerMessages::PlayerRemove { .. }
            | ServerMessages::ErrorMessage { .. } => {
                // Ces messages sont gérés par on_client_event
            }
        }
    }
}
