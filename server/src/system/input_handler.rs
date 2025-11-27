use crate::resource::server_lobby::ServerLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::client::ClientMessage;
use game_core::network::{ClientChannel, MessageDeserialize};
use game_core::player::{AimDirection, MovementController};
use game_core::projectile::{spawn_projectil, Projectile};
use game_core::server::ServerMessages;

pub fn process_client_inputs(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    lobby: Res<ServerLobby>,
    mut players: Query<(&Transform, &mut MovementController, &mut AimDirection)>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let ClientMessage::Input(input) = ClientMessage::from_bytes(&message)
                && let Some(player_entity) = lobby.get_player(&client_id)
                && let Ok((transform, mut controller, mut aim_direction)) =
                    players.get_mut(*player_entity)
            {
                let mut movement = Vec2::ZERO;

                if input.up {
                    movement.y += 1.0;
                }
                if input.down {
                    movement.y -= 1.0;
                }
                if input.left {
                    movement.x -= 1.0;
                }
                if input.right {
                    movement.x += 1.0;
                }

                if input.shoot {
                    handle_shoot(
                        transform.translation,
                        *aim_direction,
                        &mut server,
                        &mut commands,
                        &mut None,
                        &mut None,
                    );
                }

                controller.target_intent = movement.normalize_or_zero();
                aim_direction.0 = input.aim_direction;
            }
        }
    }
}

pub fn handle_shoot(
    position: Vec3,
    aim_direction: AimDirection,
    server: &mut ResMut<RenetServer>,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) {
    let server_entity = spawn_projectil(
        &Projectile,
        position,
        aim_direction,
        commands,
        meshes,
        materials,
    );

    ServerMessages::broadcast(
        &ServerMessages::ProjectileSpawned {
            server_entity,
            position,
            direction: aim_direction.0,
        },
        server,
    );
}

pub fn interpolate_movement_intent(
    time: Res<Time>,
    mut controllers: Query<&mut MovementController>,
) {
    let delta = time.delta_secs();

    for mut controller in &mut controllers {
        let distance = controller.intent.distance(controller.target_intent);

        let interpolation_speed = if distance < 0.05 {
            40.0
        } else if distance < 0.3 {
            60.0
        } else {
            100.0
        };

        let t = (interpolation_speed * delta).min(1.0);
        controller.intent = controller.intent.lerp(controller.target_intent, t);
    }
}

pub fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut avian2d::prelude::LinearVelocity)>,
) {
    const ACCELERATION: f32 = 30.0;

    const MIN_VELOCITY: f32 = 0.2;

    let delta = time.delta_secs();

    for (controller, mut velocity) in &mut movement_query {
        let desired_velocity = controller.intent * controller.max_speed;

        let t = (ACCELERATION * delta).min(1.0);
        let new_velocity = velocity.0.lerp(desired_velocity, t);

        velocity.0 = if new_velocity.length() < MIN_VELOCITY && desired_velocity.length() < 0.1 {
            Vec2::ZERO
        } else {
            new_velocity
        };
    }
}
