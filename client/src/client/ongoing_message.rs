use crate::client::enemy_event_handler::enemy_message;
use crate::client::player_event_handler::player_message;
use crate::client::projectil_event_handler::projectile_message;
use crate::game::level::Level;
use crate::resource::{ClientLobby, CurrentClientId};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::server::ServerReliableMessages;

pub fn on_client_event(
    current_client_id: Option<Res<CurrentClientId>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut commands: Commands,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
    level_query: Query<Entity, With<Level>>,
) {
    let Some(_current_client_id) = current_client_id else {
        return;
    };

    while let Some(event) = client.receive_message(ServerChannel::EntityEvent) {
        match ServerReliableMessages::from_bytes(&event) {
            ServerReliableMessages::PlayerEvent(message) => {
                player_message(
                    message,
                    &mut lobby,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &level_query,
                );
            }
            ServerReliableMessages::EnemyEvent(message) => {
                enemy_message(
                    message,
                    &mut lobby,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
            }
            ServerReliableMessages::ProjectileEvent(message) => {
                projectile_message(
                    message,
                    &mut lobby,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
            }
            ServerReliableMessages::ErrorMessage { reason } => {
                warn!("Received error message from server: {}", reason);
            }
        }
    }
}
