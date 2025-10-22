use bevy::prelude::Entity;
use bevy_renet::renet::{ChannelConfig, SendType};
use std::time::Duration;

/// Informations liant l'entité côté client à l'entité correspondante côté serveur.
///
/// - `client_entity` : entité locale représentant le joueur dans le client.
/// - `server_entity` : entité correspondante telle qu'identifiée par le serveur.
#[derive(Debug)]
pub struct PlayerEntities {
    pub client_entity: Entity,
    pub server_entity: Entity,
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
