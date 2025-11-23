use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

/// Lobby du serveur : associe les clients connectés à leurs entités.
#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    /// Mappe l'identifiant réseau d'un client à son entité Bevy
    pub players: HashMap<ClientId, Entity>,
}

impl ServerLobby {
    /// Ajoute un joueur au lobby.
    pub fn add_player(&mut self, client_id: &ClientId, entity: Entity) {
        self.players.insert(*client_id, entity);
    }

    /// Supprime un joueur du lobby.
    pub fn remove_player(&mut self, client_id: &ClientId) {
        self.players.remove(client_id);
    }

    /// Récupère l'entité associée à un client.
    pub fn get_player(&self, client_id: &ClientId) -> Option<&Entity> {
        self.players.get(client_id)
    }
}
