use crate::game::level::Level;
use crate::game::player::player;
use crate::resource::{ClientLobby, PlayerEntities};
use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use game_core::enemy::{spawn_enemy, EnemyServerEntity, EnemyType};
use game_core::player::AimDirection;
use game_core::projectile::{spawn_projectil, Projectile};
use tracing::info;

pub fn player_create(
    client_id: ClientId,
    entity: Entity,
    position: Vec3,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
    level_query: &Query<Entity, With<Level>>,
) {
    if lobby.get_player_entities(&client_id).is_none()
        && lobby.get_player_by_server_entity(&entity).is_none()
        && let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut())
        && let Ok(level_entity) = level_query.single()
    {
        let mut player_entity_id = None;
        commands.entity(level_entity).with_children(|parent| {
            let entity_commands =
                parent.spawn(player(client_id, position, 400., materials, meshes));
            player_entity_id = Some(entity_commands.id());
        });

        if let Some(player_id) = player_entity_id {
            lobby.add_player(
                &client_id,
                PlayerEntities {
                    server_entity: entity,
                    client_entity: player_id,
                },
            );
            info!("Player created: {client_id} at {position:?} with entity {entity}");
        }
    }
}

pub fn player_remove(client_id: ClientId, lobby: &mut ClientLobby, commands: &mut Commands) {
    if let Some(PlayerEntities {
        server_entity: _server_entity,
        client_entity,
    }) = lobby.remove_player(&client_id)
    {
        commands.entity(client_entity).despawn();
    }
    info!("Player removed: {client_id}");
}

pub fn enemy_spawned(
    server_entity: Entity,
    enemy_type: EnemyType,
    position: Vec3,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) {
    let e1 = spawn_enemy(commands, position, enemy_type, meshes, materials);
    commands
        .entity(e1)
        .insert(EnemyServerEntity { server_entity: e1 });
    lobby.add_enemy(server_entity, e1);
}

pub fn enemy_death(server_entity: Entity, lobby: &mut ClientLobby, commands: &mut Commands) {
    if let Some(client_entity) = lobby.remove_enemy(&server_entity) {
        commands.entity(client_entity).despawn();
    }
}

pub fn projectile_spawned(
    server_entity: Entity,
    damage: u32,
    position: Vec3,
    direction: f32,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) {
    let projectil_entity = spawn_projectil(
        &Projectile {
            damage,
            owner: server_entity,
        },
        position,
        AimDirection(direction),
        commands,
        meshes,
        materials,
    );
    lobby.add_projectile(server_entity, projectil_entity);
}

pub fn projectile_collision(
    server_entity: Entity,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
) {
    projectile_despawned(server_entity, lobby, commands);
}

pub fn projectile_cleanup(server_entity: Entity, lobby: &mut ClientLobby, commands: &mut Commands) {
    projectile_despawned(server_entity, lobby, commands);
}

fn projectile_despawned(server_entity: Entity, lobby: &mut ClientLobby, commands: &mut Commands) {
    if let Some(client_entity) = lobby.remove_projectile(&server_entity) {
        commands.entity(client_entity).despawn();
    }
}
