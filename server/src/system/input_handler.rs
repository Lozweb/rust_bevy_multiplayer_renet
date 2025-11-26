use crate::resource::server_lobby::ServerLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::client::ClientMessage;
use game_core::network::{ClientChannel, MessageDeserialize};
use game_core::player::{AimDirection, MovementController, PlayerInfo};

/// Système qui reçoit et traite les inputs de tous les clients connectés.
pub fn process_client_inputs(
    mut server: ResMut<RenetServer>,
    lobby: Res<ServerLobby>,
    mut players: Query<(&PlayerInfo, &mut MovementController, &mut AimDirection)>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let ClientMessage::Input(input) = ClientMessage::from_bytes(&message)
                && let Some(player_entity) = lobby.get_player(&client_id)
                && let Ok((_player_info, mut controller, mut aim_direction)) =
                    players.get_mut(*player_entity)
            {
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

                controller.target_intent = movement.normalize_or_zero();
                aim_direction.0 = input.aim_direction;
            }
        }
    }
}

/// Interpole l'intent du MovementController pour des mouvements plus fluides.
///
/// Ce système adoucit les changements de direction brusques causés par les inputs réseau.
/// Utilise une interpolation adaptative qui accélère quand la différence est grande.
pub fn interpolate_movement_intent(
    time: Res<Time>,
    mut controllers: Query<&mut MovementController>,
) {
    let delta = time.delta_secs();

    for mut controller in &mut controllers {
        // Distance entre l'intent actuel et la cible
        let distance = controller.intent.distance(controller.target_intent);

        // Interpolation adaptative optimisée : plus rapide et plus fluide
        // - Proche (< 0.05) : 40.0x/sec (très rapide, changements mineurs)
        // - Moyen (0.05-0.3) : 60.0x/sec (rapide, réactif)
        // - Loin (> 0.3) : 100.0x/sec (instantané, gros changements)
        let interpolation_speed = if distance < 0.05 {
            40.0
        } else if distance < 0.3 {
            60.0
        } else {
            100.0
        };

        let t = (interpolation_speed * delta).min(1.0);
        controller.intent = controller.intent.lerp(controller.target_intent, t);
    }
}

/// Applique le mouvement basé sur le MovementController.
///
/// Utilise une approche basée sur des forces au lieu de modifier directement la vélocité
/// pour permettre au moteur physique de gérer correctement les collisions.
pub fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut avian2d::prelude::LinearVelocity)>,
) {
    // Accélération augmentée pour plus de réactivité
    const ACCELERATION: f32 = 30.0;

    // Seuil de vélocité minimum réduit pour des arrêts plus naturels
    const MIN_VELOCITY: f32 = 0.2;

    let delta = time.delta_secs();

    for (controller, mut velocity) in &mut movement_query {
        let desired_velocity = controller.intent * controller.max_speed;

        // Interpolation progressive vers la vélocité désirée
        // IMPORTANT: Ne pas écraser brutalement pour permettre aux collisions
        // de fonctionner correctement
        let t = (ACCELERATION * delta).min(1.0);
        let new_velocity = velocity.0.lerp(desired_velocity, t);

        // Appliquer un seuil minimum pour arrêter complètement le mouvement
        // si la vélocité est très faible (évite les glissements infinis)
        velocity.0 = if new_velocity.length() < MIN_VELOCITY && desired_velocity.length() < 0.1 {
            Vec2::ZERO
        } else {
            new_velocity
        };
    }
}
