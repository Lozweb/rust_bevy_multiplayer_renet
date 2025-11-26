use crate::network::{MessageSerialize, ServerChannel};
use bevy::prelude::{info, Component, Entity, ResMut, Vec2, Vec3};
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
    /// Notifie le spawn d'un ennemi
    EnemySpawned {
        /// Entité de l'ennemi
        server_entity: Entity,
        /// Position de spawn
        position: Vec3,
    },
    /// Position actuelle de tous les ennemis (snapshot)
    EnemyPositions(Vec<(Entity, Vec3)>),
    /// Message d'erreur de désérialisation
    ErrorMessage { reason: String },
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
    pub fn broadcast(server_message: &ServerMessages, server: &mut ResMut<RenetServer>) {
        server.broadcast_message(
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        info!("Broadcasted: {:?}", server_message);
    }

    /// Envoie un message à un client spécifique.
    pub fn send(
        client_id: &ClientId,
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
    ) {
        server.send_message(
            *client_id,
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        info!("Send: {:?}", server_message);
    }

    /// Journalise la connexion d'un client.
    pub fn client_logon(client_id: &ClientId) {
        info!("Sending ClientLogon : Client {} connected", client_id);
    }

    /// Journalise la déconnexion d'un client.
    pub fn client_logoff(client_id: &ClientId) {
        info!("Sending ClientLogoff : Client {} disconnected", client_id);
    }
}
