use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

/// Ressource du serveur représentant le lobby.
///
/// Contient la table d'association des clients connectés vers leur entité Bevy.
/// - `players` : mappe chaque `ClientId` (identifiant réseau) à l'Entity` correspondante.
/// Cette ressource est insérée dans l'App pour suivre les joueurs connectés.
#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    /// Mappe l'identifiant réseau d'un client à son entité Bevy.
    pub players: HashMap<ClientId, Entity>,
}

impl ServerLobby {
    /// Ajoute un joueur au `ServerLobby`.
    ///
    /// Associe l'`Entity` Bevy fourni à l'`ClientId`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant réseau du client.
    /// * `entity` - Entité Bevy correspondant au joueur.
    pub fn add_player(&mut self, client_id: &ClientId, entity: Entity) {
        self.players.insert(*client_id, entity);
    }

    /// Supprime un joueur du `ServerLobby`.
    ///
    /// Retire l'association de l'`Entity` Bevy pour l'`ClientId` donné.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant réseau du client à retirer.
    pub fn remove_player(&mut self, client_id: &ClientId) {
        self.players.remove(client_id);
    }

    /// Récupère l'entité Bevy associée à un `ClientId`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Identifiant réseau du client.
    /// # Retourne
    /// * `Option<&Entity>` - Entité Bevy si trouvée, sinon `None`.
    pub fn get_player(&self, client_id: &ClientId) -> Option<&Entity> {
        self.players.get(client_id)
    }
}
