use bevy::prelude::Resource;

pub mod server_lobby;

#[derive(Resource, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}
