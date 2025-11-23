use crate::game::player::{DOWN, JUMP, LEFT, RIGHT, SHOOT, UP};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::client::ClientMessage;
use game_core::network::{ClientChannel, MessageSerialize};
use game_core::player::{AimDirection, PlayerInput};

/// Système qui enregistre les entrées locales et les envoie au serveur.
///
/// Optimisé pour n'envoyer que lorsque les inputs changent ou périodiquement
/// pour maintenir la synchronisation.
pub fn input_sync_system(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    aim_direction: Res<AimDirection>,
    mut client: ResMut<RenetClient>,
) {
    let previous_input = *player_input;

    player_input.up = keyboard_input.any_pressed(UP);
    player_input.down = keyboard_input.any_pressed(DOWN);
    player_input.left = keyboard_input.any_pressed(LEFT);
    player_input.right = keyboard_input.any_pressed(RIGHT);
    player_input.jump = keyboard_input.just_pressed(JUMP);
    player_input.shoot = mouse_input.just_pressed(SHOOT);
    player_input.aim_direction = aim_direction.0;

    // Détecte si les inputs de mouvement ont changé
    let movement_changed = previous_input.up != player_input.up
        || previous_input.down != player_input.down
        || previous_input.left != player_input.left
        || previous_input.right != player_input.right;

    // Détecte si la direction de visée a changé significativement (> 0.01 rad ~= 0.57°)
    let aim_changed = (previous_input.aim_direction - player_input.aim_direction).abs() > 0.01;

    // Envoie si : mouvement changé, visée changée, action (jump/shoot)
    let should_send = movement_changed || aim_changed || player_input.jump || player_input.shoot;

    if should_send {
        client.send_message(
            ClientChannel::Input,
            ClientMessage::to_bytes(&ClientMessage::Input(*player_input)),
        );
    }
}
