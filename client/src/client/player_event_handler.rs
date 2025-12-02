use crate::client::client_event::{player_create, player_remove};
use crate::game::level::Level;
use crate::menu::Menu;
use crate::resource::{ClientLobby, CurrentClientId};
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::{info, ColorMaterial, Commands, Entity, NextState, Query, Res, ResMut, With};
use game_core::server::PlayerMessages;

pub fn player_message(
    message: PlayerMessages,
    lobby: &mut ClientLobby,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
    level_query: &Query<Entity, With<Level>>,
    current_client_id: &Res<CurrentClientId>,
    next_menu: &mut ResMut<NextState<Menu>>,
) {
    match message {
        PlayerMessages::PlayerCreate {
            server_entity,
            client_id,
            position,
        } => player_create(
            client_id,
            server_entity,
            position,
            lobby,
            commands,
            meshes,
            materials,
            level_query,
        ),
        PlayerMessages::PlayerRemove { client_id } => player_remove(client_id, lobby, commands),
        PlayerMessages::PlayerDamaged { .. } => {
            // TODO: Afficher barre de vie, effet visuel de dégâts
        }
        PlayerMessages::PlayerDeath { player_entity } => {
            if let Some((client_id, _)) = lobby.get_player_by_server_entity(&player_entity) {
                if *client_id == current_client_id.0 {
                    info!("💀 Local player died! Showing Game Over screen");
                    next_menu.set(Menu::GameOver);
                } else {
                    info!("💀 Remote player {:?} died", player_entity);
                }
            }
        }
    }
}
