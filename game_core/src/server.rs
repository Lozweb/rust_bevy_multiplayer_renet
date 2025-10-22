use bevy::prelude::Component;
use bevy_renet::renet::{ChannelConfig, ClientId, SendType};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canal utilisé par le serveur pour envoyer des paquets au client.
///
/// - `ServerMessages` : messages serveur généraux (notifications, états de connexion).
/// - `NetworkedEntities` : mises à jour de l'état des entités réseau (positions, snapshots).
pub enum ServerChannel {
    /// Messages généraux du serveur.
    ServerMessages,
    /// Mises à jour des entités synchronisées.
    NetworkedEntities,
}

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
        entity: u64,
        id: ClientId,
        translation: [f32; 3],
    },
    /// Supprime un joueur côté client.
    ///
    /// - `id` : identifiant unique du client à retirer.
    PlayerRemove {
        id: ClientId,
    },
    Error {
        message: String,
    },
}

impl From<ServerChannel> for u8 {
    /// Convertit un `ServerChannel` en identifiant de canal ('u8').
    ///
    /// Les valeurs retournées sont utilisées par `bevy_renet` pour configurer
    /// les canaux réseau. Ce mappage doit rester cohérent avec le client.
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    /// Renvoie la configuration des canaux réseau employés par le serveur.
    ///
    /// - `NetworkedEntities` : canal non fiable pour les snapshots et mises à jour d'entités.
    /// - `ServerMessages` : canal fiable et ordonné pour les messages de contrôle
    ///   (création/suppression de joueurs, notifications).
    ///
    /// Les paramètres ('max_memory_usage_bytes', `send_type', `resend_time', ...) peuvent
    /// être ajustés selon les besoins de performance et de fiabilité.
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: ServerChannel::NetworkedEntities.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: ServerChannel::ServerMessages.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}
