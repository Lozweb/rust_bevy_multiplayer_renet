use crate::debug_state::{Log, MessageDirection};
use crate::network::{MessageSerialize, ServerChannel};
use bevy::prelude::{Component, Entity, ResMut, Vec2, Vec3};
use bevy_renet::renet::{ClientId, RenetServer};
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

/// Messages envoyés par le serveur aux clients.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    /// Notifie la création d'un joueur
    PlayerCreate {
        /// Identifiant du client propriétaire
        client_id: ClientId,
        /// Position initiale
        position: Vec3,
        /// Entité côté serveur
        entity: Entity,
    },
    /// Notifie la suppression d'un joueur
    PlayerRemove {
        /// Identifiant du client déconnecté
        client_id: ClientId,
    },
    /// Met à jour la position d'un joueur (envoyé fréquemment)
    PlayerPositionUpdate {
        /// Identifiant du joueur
        client_id: ClientId,
        /// Position actuelle
        position: Vec3,
        /// Vélocité actuelle
        velocity: Vec2,
        /// Direction de visée en radians
        aim_direction: f32,
    },
    /// Message d'erreur de désérialisation
    ErrorMessage { reason: String },
    /// Événement critiques diffusés sur le canal dédié
    CriticalEvent(CriticalServerEvent),
}

impl crate::network::DeserializeErrorFallback for ServerMessages {
    fn deserialize_error(err: DecodeError) -> Self {
        ServerMessages::ErrorMessage {
            reason: format!("Failed to deserialize ServerMessages: {}", err),
        }
    }
}

impl ServerMessages {
    /// Diffuse un message à tous les clients connectés.
    pub fn broadcast(
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
        log: &mut Option<ResMut<Log>>,
    ) {
        server.broadcast_message(
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        if let Some(log) = log {
            log.add(
                "ServerMessages".to_string(),
                MessageDirection::Sent,
                format!("Broadcasted: {:?}", server_message),
            );
        }
    }

    /// Envoie un message à un client spécifique.
    pub fn send(
        client_id: &ClientId,
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
        log: &mut Option<ResMut<Log>>,
    ) {
        server.send_message(
            *client_id,
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        if let Some(log) = log {
            log.add(
                "ServerMessages".to_string(),
                MessageDirection::Sent,
                format!("Sent to {}: {:?}", client_id, server_message),
            );
        }
    }

    /// Journalise la connexion d'un client.
    pub fn client_logon(client_id: &ClientId, log: &mut Option<ResMut<Log>>) {
        if let Some(log) = log {
            log.add(
                "ServerMessages".to_string(),
                MessageDirection::Sent,
                format!("Client {} connected", client_id),
            );
        }
    }

    /// Journalise la déconnexion d'un client.
    pub fn client_logoff(client_id: &ClientId, log: &mut Option<ResMut<Log>>) {
        if let Some(log) = log {
            log.add(
                "ServerMessages".to_string(),
                MessageDirection::Sent,
                format!("Client {} disconnected", client_id),
            );
        }
    }
}

/// Événements critiques traités immédiatement.
///
/// Ces événements sont envoyés sur un canal dédié pour un traitement
/// en temps réel, séparément des autres messages du serveur.
#[derive(Debug, Serialize, Deserialize, Component, Clone)]
pub enum CriticalServerEvent {
    /// Exemple: tir, pickup, action immédiate (future extension)
    ProjectileFired { client_id: ClientId },
}

/// Snapshot des entités réseau et leurs positions.
///
/// Les vecteurs `entities` et `translations` sont parallèles :
/// l'élément à l'index `i` dans `entities` correspond à la position
/// à l'index `i` dans `translations`.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    /// Identifiants des entités côté serveur
    pub entities: Vec<u64>,
    /// Positions des entités [x, y, z]
    pub translations: Vec<[f32; 3]>,
}
