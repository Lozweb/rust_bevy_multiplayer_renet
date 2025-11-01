use crate::player::PlayerInput;
use bevy::prelude::Component;
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

/// Représente un message envoyé par le client au serveur.
///
/// Variantes :
/// - `Input(PlayerInput)` : transmet les entrées du joueur.
/// - `Command(String)` : envoie une commande texte.
/// - `ErrorMessage { reason: String }` : signale une erreur avec une raison.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    Input(PlayerInput),
    Command(String),
    ErrorMessage { reason: String },
}

/// Implémentation du trait `DeserializeErrorFallback` pour `ClientMessage`.
/// Cette méthode permet de créer un message d'erreur structuré lorsque la
/// désérialisation d'un message client échoue, en encapsulant la raison
/// de l'échec dans la variante `ErrorMessage`.
impl crate::network::DeserializeErrorFallback for ClientMessage {
    fn deserialize_error(err: DecodeError) -> Self {
        ClientMessage::ErrorMessage {
            reason: format!("Failed to deserialize ClientMessage: {}", err),
        }
    }
}
