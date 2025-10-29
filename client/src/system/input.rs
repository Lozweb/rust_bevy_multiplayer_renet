use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Res, ResMut};
use bevy_renet::renet::RenetClient;
use game_core::client::ClientChannel;
use game_core::network::serialize_server_message;
use game_core::player::PlayerInput;

const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
const JUMP: KeyCode = KeyCode::Space;
const SHOOT: MouseButton = MouseButton::Left;

pub fn input_sync_system(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut client: ResMut<RenetClient>,
) {
    player_input.up = keyboard_input.any_pressed(UP);
    player_input.down = keyboard_input.any_pressed(DOWN);
    player_input.left = keyboard_input.any_pressed(LEFT);
    player_input.right = keyboard_input.any_pressed(RIGHT);
    player_input.jump = keyboard_input.just_pressed(JUMP);
    player_input.shoot = mouse_input.just_pressed(SHOOT);

    let message = serialize_server_message(&*player_input);
    client.send_message(ClientChannel::Input, message);
}
