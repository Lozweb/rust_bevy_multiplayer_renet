use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;
#[derive(Debug)]
pub struct PlayerEntities {
    pub client_entity: Entity,
    pub server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    pub players: HashMap<ClientId, PlayerEntities>,
    pub ennemies: HashMap<Entity, Entity>,
    pub projectiles: HashMap<Entity, Entity>,
}

impl ClientLobby {
    pub fn add_player(&mut self, client_id: &ClientId, entities: PlayerEntities) {
        self.players.insert(*client_id, entities);
    }

    pub fn remove_player(&mut self, client_id: &ClientId) -> Option<PlayerEntities> {
        self.players.remove(client_id)
    }

    pub fn get_player_by_server_entity(
        &self,
        server_entity: &Entity,
    ) -> Option<(&ClientId, &PlayerEntities)> {
        self.players.iter().find_map(|(client_id, entities)| {
            if &entities.server_entity == server_entity {
                Some((client_id, entities))
            } else {
                None
            }
        })
    }

    pub fn get_player_entities(&self, client_id: &ClientId) -> Option<&PlayerEntities> {
        self.players.get(client_id)
    }

    pub fn add_enemy(&mut self, server_entity: Entity, client_entity: Entity) {
        self.ennemies.insert(server_entity, client_entity);
    }
    pub fn remove_enemy(&mut self, server_entity: &Entity) -> Option<Entity> {
        self.ennemies.remove(server_entity)
    }

    pub fn get_enemy_entity(&self, server_entity: &Entity) -> Option<&Entity> {
        self.ennemies.get(server_entity)
    }

    pub fn add_projectile(&mut self, server_entity: Entity, client_entity: Entity) {
        self.projectiles.insert(server_entity, client_entity);
    }

    pub fn remove_projectile(&mut self, server_entity: &Entity) -> Option<Entity> {
        self.projectiles.remove(server_entity)
    }
    pub fn get_projectile_entity(&self, server_entity: &Entity) -> Option<&Entity> {
        self.projectiles.get(server_entity)
    }
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(pub ClientId);
