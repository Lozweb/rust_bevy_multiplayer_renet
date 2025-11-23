/// Représente un message envoyé par le client au serveur.
///
/// Variantes :
/// - `Input(PlayerInput)` : transmet les entrées du joueur.
/// - `Command(String)` : envoie une commande texte.
/// - `ErrorMessage { reason: String }` : signale une erreur avec une raison.
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    ClientReady,
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

use crate::player::PlayerInput;
use bevy::log::error;
use bevy::prelude::Component;
use bevy_renet::renet::{ChannelConfig, ConnectionConfig, SendType};
use bincode::error::DecodeError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

/// Identifiant de protocole utilisé pour vérifier la compatibilité client/serveur.
/// Incrémentez cette valeur à chaque changement incompatible du protocole réseau.
/// Si le client et le serveur n'ont pas le même `PROTOCOL_ID', la connexion échoue.
pub const PROTOCOL_ID: u64 = 1;

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
pub fn get_current_time() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            error!("Erreur lors de la récupération du temps système: {e}");
            Duration::from_secs(0)
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

/// Canal utilisé par le serveur pour envoyer des paquets au client.
///
/// - `ServerMessages` : messages serveur généraux (notifications, états de connexion).
/// - `NetworkedEntities` : mises à jour de l'état des entités réseau (positions, snapshots).
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerChannel {
    /// Messages généraux du serveur.
    ServerMessages,
    /// Mises à jour des entités synchronisées.
    NetworkedEntities,
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

/// Canal utilisé par le client pour envoyer des paquets au serveur.
///
/// - `Input` : envoie les entrées du joueur (contrôles, mouvements) à haute fréquence.
/// - `Command` : envoie des commandes ponctuelles (ex : chat, actions, requêtes).
pub enum ClientChannel {
    /// Entrées de contrôle du joueur.
    Input,
    /// Commandes ponctuelles et requêtes.
    Command,
}

/// Conversion de `ClientChannel` en identifiant numérique (`u8`).
///
/// Mapping explicite utilisé pour communiquer avec l'API réseau :
/// - `Command` -> 0
/// - `Input` -> 1
impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}

/// Fournit la configuration des canaux côté client.
///
/// Canaux :
/// - `Input` : envoie des entrées du joueur à haute fréquence. Configuré en `ReliableOrdered`
///   avec `resend_time = Duration::ZERO` pour faible latence et ordre garanti.
/// - `Command` : envoie des commandes ponctuelles (chat, actions). Également `ReliableOrdered'.
///
/// Les tailles mémoire sont plafonnées à 5 MiB par canal.
impl ClientChannel {
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::Command.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
        ]
    }
}

/// Trait pour la sérialisation des messages réseau.
///
/// Permet de convertir un message en un vecteur d'octets (`Vec<u8>`) prêt à être envoyé.
/// Les types qui implémentent ce trait doivent fournir une méthode `to_bytes`
/// qui effectue la sérialisation.
///
/// Généralement, ce trait est automatiquement implémenté pour les types qui dérivent `Serialize`.
pub trait MessageSerialize {
    /// Sérialise le message en un vecteur d'octets.
    fn to_bytes(&self) -> Vec<u8>;
}

/// Implémentation générique de `MessageSerialize` pour tous les types sérialisables.
///
/// Cette implémentation permet à tout type qui implémente `Serialize`
/// d'être automatiquement sérialisé en un vecteur d'octets via la méthode `to_bytes`.
impl<T> MessageSerialize for T
where
    T: Serialize,
{
    /// Sérialise l'instance en un vecteur d'octets prêt à être envoyé sur le réseau.
    fn to_bytes(&self) -> Vec<u8> {
        serialize_message(self)
    }
}

/// Trait pour la désérialisation des messages réseau.
///
/// Permet de reconstruire une instance à partir d'un tableau d'octets (`&[u8]`)
/// reçu sur le réseau. Les types qui implémentent ce trait doivent fournir une
/// méthode `from_bytes` qui effectue la désérialisation. Généralement, ce trait
/// est automatiquement implémenté pour les types qui dérivent `Deserialize` et
/// `DeserializeErrorFallback`.
pub trait MessageDeserialize: Sized {
    /// Désérialise une instance à partir d'un tableau d'octets.
    ///
    /// # Paramètres
    /// - `bytes` : données binaires à désérialiser.
    ///
    /// # Retour
    /// Une instance du type cible, ou une valeur de repli en cas d'erreur.
    fn from_bytes(bytes: &[u8]) -> Self;
}

/// Trait permettant de définir une valeur de repli lors d'une erreur de désérialisation.
///
/// Lorsqu'une erreur de décodage (`DecodeError`) survient lors de la désérialisation
/// d'un message réseau, ce trait permet de fournir une instance par défaut ou une
/// valeur spécifique à retourner à la place.
///
/// À implémenter pour les types nécessitant une gestion personnalisée des erreurs
/// de désérialisation.
pub trait DeserializeErrorFallback {
    /// Retourne une instance de repli du type en cas d'erreur de désérialisation.
    ///
    /// # Paramètres
    /// - `err` : l'erreur de décodage rencontrée.
    fn deserialize_error(_err: DecodeError) -> Self;
}
/// Implémentation générique de `MessageDeserialize` pour tous les types qui implémentent
/// à la fois `DeserializeOwned` et `DeserializeErrorFallback`.
///
/// Cette implémentation tente de désérialiser un message à partir d'un tableau d'octets.
/// En cas d'échec, elle journalise l'erreur et retourne une valeur de repli définie
/// par `T::deserialize_error`.
impl<T> MessageDeserialize for T
where
    T: DeserializeOwned + DeserializeErrorFallback,
{
    /// Désérialise une instance à partir d'un tableau d'octets.
    ///
    /// # Paramètres
    /// - `bytes` : données binaires à désérialiser.
    ///
    /// # Retour
    /// Une instance du type cible, ou une valeur de repli en cas d'erreur.
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode::config::standard())
            .map(|(msg, _)| msg)
            .unwrap_or_else(|err| {
                error!("Deserialization error: {:?}", err);
                T::deserialize_error(err)
            })
    }
}

/// Sérialise un message réseau en un vecteur d'octets.
///
/// # Paramètres
/// - `message` : référence vers le message à sérialiser. Le type doit implémenter `MessageSerialize` et `Serialize`.
///
/// # Retour
/// Un `Vec<u8>` contenant la représentation binaire du message, prêt à être envoyé sur le réseau.
/// En cas d'erreur de sérialisation, la fonction journalise l'erreur et retourne un vecteur vide.
///
/// # Exemple
/// ```
/// let msg = MonMessage { ... };
/// let bytes = serialize_message(&msg);
/// ```
pub fn serialize_message<T: MessageSerialize>(message: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serde::encode_to_vec(message, bincode::config::standard()).unwrap_or_else(|err| {
        error!("Serialization error: {:?}", err);
        Vec::new()
    })
}
