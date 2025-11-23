use crate::resource::server_lobby::ServerLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::client::ClientMessage;
use game_core::network::{ClientChannel, MessageDeserialize};
use game_core::player::PlayerInfo;

/// Contrôleur de mouvement pour les joueurs côté serveur.
/// Identique à celui du client pour une simulation cohérente.
#[derive(Component)]
pub struct MovementController {
    pub intent: Vec2,
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            max_speed: 400.0,
        }
    }
}

/// Direction de visée du joueur en radians côté serveur.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct AimDirection(pub f32);

/// Système qui reçoit et traite les inputs de tous les clients connectés.
#[allow(clippy::type_complexity)]
pub fn process_client_inputs(
    mut server: ResMut<RenetServer>,
    lobby: Res<ServerLobby>,
    mut players: Query<(&PlayerInfo, &mut MovementController, &mut AimDirection)>,
) {
    // Pour chaque client connecté
    for client_id in server.clients_id() {
        // Lire tous les messages Input de ce client
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            // Désérialiser le message
            match ClientMessage::from_bytes(&message) {
                ClientMessage::Input(input) => {
                    // Trouver l'entité du joueur via le lobby
                    if let Some(player_entity) = lobby.get_player(&client_id) {
                        // Appliquer l'input au joueur
                        if let Ok((_player_info, mut controller, mut aim_direction)) =
                            players.get_mut(*player_entity)
                        {
                            // Calculer la direction de mouvement basée sur les inputs
                            let mut movement = Vec2::ZERO;

                            if input.up {
                                movement.y += 1.0;
                            }
                            if input.down {
                                movement.y -= 1.0;
                            }
                            if input.left {
                                movement.x -= 1.0;
                            }
                            if input.right {
                                movement.x += 1.0;
                            }

                            controller.intent = movement.normalize_or_zero();
                            aim_direction.0 = input.aim_direction;
                        }
                    }
                }

                ClientMessage::Command(_) | ClientMessage::ErrorMessage { .. } => {
                    // Ces messages ne sont pas gérés par ce système
                }
            }
        }
    }
}

/// Applique le mouvement basé sur le MovementController.
pub fn apply_movement(
    mut movement_query: Query<(&MovementController, &mut avian2d::prelude::LinearVelocity)>,
) {
    for (controller, mut velocity) in &mut movement_query {
        velocity.0 = controller.intent * controller.max_speed;
    }
}
