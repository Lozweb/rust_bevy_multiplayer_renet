use crate::client::ClientChannel;
use crate::server::{ServerChannel, ServerMessages};
use bevy::log::error;
use bevy_renet::renet::ConnectionConfig;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

/// Identifiant de protocole utilisé pour vérifier la compatibilité client/serveur.
/// Incrémentez cette valeur à chaque changement incompatible du protocole réseau.
/// Si le client et le serveur n'ont pas le même `PROTOCOL_ID', la connexion échoue.
pub const PROTOCOL_ID: u64 = 1;

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

/// Retourne la configuration de connexion utilisée par renet.
///
/// - `available_bytes_per_tick` : bande passante maximale autorisée par tick (en octets).
/// - `client_channels_config` : configuration des canaux côté client.
/// - `server_channels_config` : configuration des canaux côté serveur.
///
/// Cette configuration est partagée par le client et le serveur lors de la création
/// d'une connexion renet.
pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channel_config(),
        server_channels_config: ServerChannel::channel_config(),
    }
}

/// Retourne la durée écoulée depuis `UNIX_EPOCH'.
///
/// En cas d'erreur (par exemple si l'horloge système est avant epoch),
/// la fonction journalise l'erreur via 'bevy::log::error' et retourne une durée nulle.
pub fn get_current_time() -> std::time::Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            error!("Erreur lors de la récupération du temps système: {e}");
            std::time::Duration::from_secs(0)
        })
}

/// Crée et retourne un `UdpSocket` lié à l'adresse fournie.
///
/// # Paramètres
///
/// - `socket_address` : adresse locale à lier.
///
/// # Panique
///
/// Panique si 'UdpSocket::bind' échoue (erreur renvoyée incluse dans le message).
pub fn get_socket(socket_address: SocketAddr) -> UdpSocket {
    match UdpSocket::bind(socket_address) {
        Ok(s) => s,
        Err(e) => panic!("Erreur lors de la création du socket UDP: {e}"),
    }
}

pub fn deserialize_server_message(message: &[u8]) -> (ServerMessages, usize) {
    bincode::serde::decode_from_slice(message, bincode::config::standard()).unwrap_or_else(|err| {
        error!("Deserialization error: {:?}", err);
        (
            ServerMessages::Error {
                message: err.to_string(),
            },
            0,
        )
    })
}

pub fn serialize_server_message(message: &ServerMessages) -> Vec<u8> {
    bincode::serde::encode_to_vec(message, bincode::config::standard()).unwrap_or_else(|err| {
        error!("Serialization error: {:?}", err);
        Vec::new()
    })
}
