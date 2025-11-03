use crate::debug_state::{Log, MessageDirection};
use crate::network::{MessageSerialize, ServerChannel};
use bevy::prelude::{Component, Entity, ResMut, Vec3};
use bevy_renet::renet::{ClientId, RenetServer};
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

/// Messages envoyés par le serveur aux clients.
///
/// Ces messages sont sérialisés via `serde` et transmis sur les canaux définis
/// dans `ServerChannel'.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    /// Crée un joueur côté client.
    ///
    /// - `entity` : identifiant de l'entité côté serveur (permets le mapping).
    /// - `id` : identifiant unique du client ('ClientId').
    /// - `translation` : position initiale du joueur sous la forme `[x, y, z]'.
    PlayerCreate {
        client_id: ClientId,
        position: Vec3,
        entity: Entity,
    },
    /// Supprime un joueur côté client.
    ///
    /// - `id` : identifiant unique du client à retirer.
    PlayerRemove {
        client_id: ClientId,
    },
    ErrorMessage {
        reason: String,
    },
}

/// Implémentation du trait `DeserializeErrorFallback` pour `ServerMessages`.
///
/// Cette implémentation permet de gérer les erreurs de désérialisation.
/// Lorsqu'une erreur de décodage survient lors de la réception d'un message du serveur,
/// un message `ErrorMessage` est généré avec la raison de l'échec.
///
/// # Arguments
///
/// * `err` - L'erreur de décodage rencontrée lors de la désérialisation.
///
/// # Retour
///
/// Retourne une variante `ErrorMessage` contenant la description de l'erreur.
impl crate::network::DeserializeErrorFallback for ServerMessages {
    fn deserialize_error(err: DecodeError) -> Self {
        ServerMessages::ErrorMessage {
            reason: format!("Failed to deserialize ServerMessages: {}", err),
        }
    }
}

impl ServerMessages {
    /// Envoie un message à tous les clients connectés.
    ///
    /// # Arguments
    ///
    /// * `server_message` - Référence vers le message à envoyer à tous les clients.
    /// * `server` - Référence mutable vers le serveur Renet.
    /// * `log` - Référence mutable vers le journal des messages.
    ///
    /// Le message est sérialisé et envoyé sur le canal `ServerMessages` à tous les clients.
    pub fn broadcast(
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
        log: &mut ResMut<Log>,
    ) {
        server.broadcast_message(
            ServerChannel::ServerMessages,
            ServerMessages::to_bytes(server_message),
        );
        log.add(
            "ServerMessages".to_string(),
            MessageDirection::Sent,
            format!("Broadcasted: {:?}", server_message),
        )
    }

    /// Envoie un message du serveur à un client spécifique.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant du client destinataire.
    /// * `server_message` - Message à envoyer.
    /// * `server` - Référence mutable vers le serveur Renet.
    /// * `log` - Référence mutable vers le journal des messages.
    ///
    /// Le message est sérialisé et envoyé sur le canal `ServerMessages` au client spécifié.
    pub fn send(
        client_id: &ClientId,
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
        log: &mut ResMut<Log>,
    ) {
        server.send_message(
            *client_id,
            ServerChannel::ServerMessages,
            ServerMessages::to_bytes(server_message),
        );
        log.add(
            "ServerMessages".to_string(),
            MessageDirection::Sent,
            format!("Sent to {}: {:?}", client_id, server_message),
        );
    }

    /// Journalise la connexion d'un client.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant du client qui vient de se connecter.
    /// * `log` - Référence mutable vers le journal des messages.
    pub fn client_logon(client_id: &ClientId, log: &mut ResMut<Log>) {
        log.add(
            "ServerMessages".to_string(),
            MessageDirection::Sent,
            format!("Client {} connected", client_id),
        );
    }
    /// Journalise la déconnexion d'un client.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant du client qui vient de se déconnecter.
    /// * `log` - Référence mutable vers le journal des messages.
    pub fn client_logoff(client_id: &ClientId, log: &mut ResMut<Log>) {
        log.add(
            "ServerMessages".to_string(),
            MessageDirection::Sent,
            format!("Client {} disconnected", client_id),
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// Représente un snapshot des entités synchronisées et leurs positions.
///
/// Cette structure contient deux vecteurs parallèles :
/// - `entities` : identifiants uniques des entités côté serveur ('u64').
/// - `translations` : positions sous la forme `[x, y, z]` pour chaque entité.
///
/// Contrat : les deux vecteurs doivent avoir la même longueur. L'élément à l'index `i`
/// dans `entities` correspond à la position à l'index `i` dans `translations'.
///
/// Sérialisée via `serde` pour être envoyée sur le canal `NetworkedEntities'.
pub struct NetworkedEntities {
    /// Identifiants des entités côté serveur.
    pub entities: Vec<u64>,
    /// Positions des entités : `[x, y, z]`.
    pub translations: Vec<[f32; 3]>,
}
