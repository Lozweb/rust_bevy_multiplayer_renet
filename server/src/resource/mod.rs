use bevy::prelude::Resource;

pub mod server_lobby;

/// Configuration du serveur.
#[derive(Resource, Clone)]
pub struct ServerConfig {
    /// Port d'écoute
    pub port: u16,
}

impl ServerConfig {
    /// Crée une nouvelle configuration serveur.
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}
