use crate::network::DeserializeErrorFallback;
use crate::player::PlayerInput;
use bevy::prelude::Component;
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

/// Messages envoyés par le client au serveur.
///
/// # Variantes
///
/// * `Input` - Transmet les entrées du joueur (mouvement, visée, actions)
/// * `Command` - Envoie une commande texte au serveur
/// * `ErrorMessage` - Signale une erreur de désérialisation
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    /// Entrées du joueur
    Input(PlayerInput),
    /// Commande texte
    Command(String),
    /// Message d'erreur avec raison
    ErrorMessage { reason: String },
}

impl DeserializeErrorFallback for ClientMessage {
    fn deserialize_error(err: DecodeError) -> Self {
        ClientMessage::ErrorMessage {
            reason: format!("Failed to deserialize ClientMessage: {}", err),
        }
    }
}
