use bevy::log::error;
use bevy::prelude::Component;
use bevy_renet::renet::{ChannelConfig, ConnectionConfig, SendType};
use bincode::error::DecodeError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

/// Identifiant de protocole pour la compatibilité client/serveur.
///
/// Incrémentez cette valeur à chaque changement incompatible du protocole réseau.
pub const PROTOCOL_ID: u64 = 1;

/// Retourne la configuration de connexion renet.
///
/// Configure la bande passante maximale (1 MiB/tick) et les canaux client/serveur.
pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channel_config(),
        server_channels_config: ServerChannel::channel_config(),
    }
}

/// Retourne la durée écoulée depuis `UNIX_EPOCH`.
///
/// En cas d'erreur (horloge système avant epoch), journalise et retourne `Duration::ZERO`.
pub fn get_current_time() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            error!("Erreur lors de la récupération du temps système: {e}");
            Duration::from_secs(0)
        })
}

/// Crée un `UdpSocket` lié à l'adresse fournie.
///
/// # Panics
///
/// Panique si `UdpSocket::bind` échoue.
pub fn get_socket(socket_address: SocketAddr) -> UdpSocket {
    UdpSocket::bind(socket_address)
        .unwrap_or_else(|e| panic!("Erreur lors de la création du socket UDP: {e}"))
}

/// Canaux réseau utilisés par le serveur pour envoyer des messages aux clients.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerChannel {
    /// Snapshots de position et d'état (unreliable, haute fréquence)
    Snapshots,
    /// Événements critiques de gameplay (reliable, faible latence)
    CriticalEvents,
    /// Messages de contrôle fiables (spawn/despawn, erreurs)
    ReliableState,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::Snapshots => 0,
            ServerChannel::CriticalEvents => 1,
            ServerChannel::ReliableState => 2,
        }
    }
}

impl ServerChannel {
    /// Configuration des canaux serveur.
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: ServerChannel::Snapshots.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: ServerChannel::CriticalEvents.into(),
                max_memory_usage_bytes: 2 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(100),
                },
            },
            ChannelConfig {
                channel_id: ServerChannel::ReliableState.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(250),
                },
            },
        ]
    }
}

/// Canaux réseau utilisés par le client pour envoyer des messages au serveur.
pub enum ClientChannel {
    /// Entrées du joueur (haute fréquence)
    Input,
    /// Commandes fiables ponctuelles (chat, interactions critiques)
    ReliableCommand,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Input => 0,
            ClientChannel::ReliableCommand => 1,
        }
    }
}

impl ClientChannel {
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: ClientChannel::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: ClientChannel::ReliableCommand.into(),
                max_memory_usage_bytes: 2 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}

/// Trait pour sérialiser des messages réseau en octets.
pub trait MessageSerialize {
    /// Sérialise le message en `Vec<u8>`.
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T> MessageSerialize for T
where
    T: Serialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        serialize_message(self)
    }
}

/// Trait pour désérialiser des messages réseau depuis des octets.
pub trait MessageDeserialize: Sized {
    /// Désérialise une instance depuis `&[u8]`.
    ///
    /// Retourne une valeur de repli en cas d'erreur de désérialisation.
    fn from_bytes(bytes: &[u8]) -> Self;
}

/// Trait définissant une valeur de repli en cas d'erreur de désérialisation.
pub trait DeserializeErrorFallback {
    /// Retourne une instance de repli lors d'une erreur de désérialisation.
    fn deserialize_error(err: DecodeError) -> Self;
}

impl<T> MessageDeserialize for T
where
    T: DeserializeOwned + DeserializeErrorFallback,
{
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode::config::standard())
            .map(|(msg, _)| msg)
            .unwrap_or_else(|err| {
                error!("Deserialization error: {:?}", err);
                T::deserialize_error(err)
            })
    }
}

/// Sérialise un message en vecteur d'octets via bincode.
///
/// Retourne un vecteur vide en cas d'erreur de sérialisation (avec log).
pub fn serialize_message<T: MessageSerialize>(message: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serde::encode_to_vec(message, bincode::config::standard()).unwrap_or_else(|err| {
        error!("Serialization error: {:?}", err);
        Vec::new()
    })
}
