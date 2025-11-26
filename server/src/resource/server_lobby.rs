use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}

impl ServerLobby {
    pub fn add_player(&mut self, client_id: &ClientId, entity: Entity) {
        self.players.insert(*client_id, entity);
    }

    pub fn remove_player(&mut self, client_id: &ClientId) {
        self.players.remove(client_id);
    }

    pub fn get_player(&self, client_id: &ClientId) -> Option<&Entity> {
        self.players.get(client_id)
    }
}
