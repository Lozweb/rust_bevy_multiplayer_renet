use crate::game::player::{AimDirection, DOWN, JUMP, LEFT, RIGHT, SHOOT, UP};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Res, ResMut};
use bevy_renet::renet::RenetClient;
use game_core::client::{ClientMessage, MessageSerialize};
use game_core::network::ClientChannel;
use game_core::player::PlayerInput;

pub fn input_sync_system(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    aim_direction: Res<AimDirection>,
    mut client: ResMut<RenetClient>,
) {
    player_input.up = keyboard_input.any_pressed(UP);
    player_input.down = keyboard_input.any_pressed(DOWN);
    player_input.left = keyboard_input.any_pressed(LEFT);
    player_input.right = keyboard_input.any_pressed(RIGHT);
    player_input.jump = keyboard_input.just_pressed(JUMP);
    player_input.shoot = mouse_input.just_pressed(SHOOT);
    player_input.aim_direction = aim_direction.0;

    client.send_message(
        ClientChannel::Input,
        ClientMessage::to_bytes(&ClientMessage::Input(*player_input)),
    );
}
