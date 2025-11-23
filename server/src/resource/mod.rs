use bevy::prelude::Resource;

pub mod server_lobby;

#[derive(Resource, Clone)]
pub struct ServerConfig {
    pub headless: bool,
    pub port: u16,
}

impl ServerConfig {
    pub fn new(headless: bool, port: u16) -> Self {
        Self { headless, port }
    }
}
