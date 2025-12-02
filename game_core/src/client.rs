use crate::network::DeserializeErrorFallback;
use crate::player::PlayerInput;
use bevy::prelude::Component;
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClientCommand {
    Respawn,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    Input(PlayerInput),
    Command(ClientCommand),
    ErrorMessage { reason: String },
}

impl DeserializeErrorFallback for ClientMessage {
    fn deserialize_error(err: DecodeError) -> Self {
        ClientMessage::ErrorMessage {
            reason: format!("Failed to deserialize ClientMessage: {}", err),
        }
    }
}
