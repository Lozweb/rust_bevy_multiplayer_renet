use crate::network::DeserializeErrorFallback;
use crate::player::PlayerInput;
use bevy::prelude::Component;
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    Input(PlayerInput),
    Command(String),
    ErrorMessage { reason: String },
}

impl DeserializeErrorFallback for ClientMessage {
    fn deserialize_error(err: DecodeError) -> Self {
        ClientMessage::ErrorMessage {
            reason: format!("Failed to deserialize ClientMessage: {}", err),
        }
    }
}
