//! Comportement spécifique au joueur.
//!
//! Ce module gère le joueur, son viseur (aim rig) et les entrées de direction.
//! Il fournit les composants et ressources nécessaires pour contrôler le personnage
//! du joueur et gérer sa visée avec la souris ou la manette.

use crate::client::input::input_sync_system;
use crate::client::position_sync::NetworkedTransform;
use crate::client::Connected;
use crate::game::camera::MainCamera;
use crate::{asset_tracking::LoadResource, AppSystems, PausableSystems};
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, DebugRender, LinearDamping, LinearVelocity, LockedAxes, Mass,
    RigidBody,
};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_renet::client_connected;
use bevy_renet::renet::ClientId;
use game_core::player::{
    AimDirection, ControlledPlayer, MouseWorldCoords, MovementController, PlayerInfo, PlayerInput,
};

pub(crate) const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
pub(crate) const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
pub(crate) const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub(crate) const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
pub(crate) const AIM_RADIUS: f32 = 75.;
pub(crate) const JUMP: KeyCode = KeyCode::Space;
pub(crate) const SHOOT: MouseButton = MouseButton::Left;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// Source de la direction de visée.
///
/// Détermine si la visée provient de la souris ou d'une manette de jeu.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect)]
pub enum AimSource {
    /// Visée contrôlée par la souris.
    Mouse,
    /// Visée contrôlée par la manette.
    Gamepad,
}

/// Composant de viseur pour le personnage joueur.
///
/// Gère le cercle de visée et la croix de visée qui suivent la direction de la souris.
/// Le viseur tourne autour du joueur pour indiquer où le joueur vise.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AimRig {
    /// Le rayon du cercle de visée en pixels.
    pub radius: f32,

    /// La source de la direction de visée (souris ou manette).
    pub source: AimSource,
}

/// Bundle regroupant les composants physiques du joueur LOCAL uniquement.
/// Les joueurs distants n'ont PAS de physique côté client pour éviter la désynchronisation.
#[derive(Bundle)]
struct LocalPlayerPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    mass: Mass,
    linear_damping: LinearDamping,
    linear_velocity: LinearVelocity,
    locked_axes: LockedAxes,
    collision_events: CollisionEventsEnabled,
    debug_render: DebugRender,
}

impl Default for LocalPlayerPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(32.0, 32.0),
            mass: Mass(50.0),
            linear_damping: LinearDamping(1.5),
            linear_velocity: LinearVelocity::ZERO,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_events: CollisionEventsEnabled,
            debug_render: DebugRender::default().with_collider_color(Color::WHITE),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        Update,
        (record_player_directional_input, record_aim_direction)
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    app.insert_resource(PlayerInput::default());
    app.insert_resource(MouseWorldCoords::default());
    app.insert_resource(AimDirection::default());

    app.add_systems(Update, input_sync_system.in_set(Connected));
    app.add_systems(Update, mark_local_player.in_set(Connected));
    app.configure_sets(Update, Connected.run_if(client_connected));
}

/// Crée un bundle complet pour l'entité joueur.
///
/// # Arguments
///
/// * `max_speed` - Vitesse maximale de déplacement du joueur
/// * `materials` - Ressource des matériaux pour créer les visuels
/// * `meshes` - Ressource des meshes pour créer les formes
///
/// # Retour
///
/// Un bundle contenant tous les composants nécessaires au joueur :
/// - Composants de rendu (mesh, matériaux)
/// - Composants de physique (rigidbody, collider)
/// - Contrôleur de mouvement
/// - Système de visée (AimRig avec cercle et croix)
pub fn player(
    client_id: ClientId,
    position: Vec3,
    max_speed: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> impl Bundle {
    let body_mesh = meshes.add(Rectangle::new(32., 32.));
    let body_material = materials.add(ColorMaterial::from(Color::WHITE));
    let aim_circle_mesh = meshes.add(Circle::new(AIM_RADIUS));
    let aime_circle_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 1.0, 1.0, 0.2)));
    let cross_horizontal_mesh = meshes.add(Rectangle::new(16., 2.));
    let cross_vertical_mesh = meshes.add(Rectangle::new(2., 16.));
    let cross_material = materials.add(ColorMaterial::from(Color::WHITE));
    (
        Name::new("Player"),
        Player,
        PlayerInfo {
            id: client_id,
            name: format!("Player_{client_id}"),
        },
        Mesh2d(body_mesh),
        MeshMaterial2d(body_material),
        Transform::from_translation(position),
        MovementController {
            max_speed,
            ..default()
        },
        AimDirection::default(),
        Collider::rectangle(32.0, 32.0),
        NetworkedTransform::default(),
        children![(
            Name::new("AimRig"),
            Visibility::Inherited,
            Transform::default(),
            AimRig {
                radius: AIM_RADIUS,
                source: AimSource::Mouse,
            },
            children![
                (
                    Name::new("AimCircle"),
                    Visibility::Inherited,
                    Mesh2d(aim_circle_mesh),
                    MeshMaterial2d(aime_circle_material),
                    Transform::from_xyz(0.0, 0.0, -0.1),
                ),
                (
                    Name::new("AimCross"),
                    Visibility::Inherited,
                    Transform::from_translation(Vec3::new(AIM_RADIUS, 0., 0.1)),
                    children![
                        (
                            Visibility::Inherited,
                            Mesh2d(cross_horizontal_mesh),
                            MeshMaterial2d(cross_material.clone()),
                        ),
                        (
                            Visibility::Inherited,
                            Mesh2d(cross_vertical_mesh),
                            MeshMaterial2d(cross_material),
                        ),
                    ],
                )
            ]
        )],
    )
}

/// Marque automatiquement le joueur local avec le composant ControlledPlayer.
///
/// Ce système identifie les joueurs qui ont un PlayerInfo.id correspondant au
/// CurrentClientId et leur ajoute le composant ControlledPlayer ainsi que la physique Dynamic.
/// Les joueurs distants reçoivent un RigidBody::Static pour pouvoir être poussés par le local.
fn mark_local_player(
    mut commands: Commands,
    current_client_id: Res<crate::resource::CurrentClientId>,
    untagged_players: Query<(Entity, &PlayerInfo), (With<Player>, Without<ControlledPlayer>)>,
) {
    for (entity, player_info) in &untagged_players {
        if player_info.id == current_client_id.0 {
            info!("Marking player {} as locally controlled", player_info.id);
            commands
                .entity(entity)
                .insert((ControlledPlayer, LocalPlayerPhysicsBundle::default()));
        } else {
            info!("Marking player {} as remote (static)", player_info.id);
            commands.entity(entity).insert(RigidBody::Static);
        }
    }
}

fn record_aim_direction(
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    mut player_query: Query<
        (&GlobalTransform, &mut AimDirection),
        (With<Player>, With<ControlledPlayer>),
    >,
    mut aim_direction_resource: ResMut<AimDirection>,
) {
    // Si aucun joueur contrôlé n'existe, ne rien faire
    let Some((player_transform, mut aim_direction)) = player_query.iter_mut().next() else {
        return;
    };

    let mouse_coords = window.cursor_position().map(|pos| {
        let (camera, camera_transform) = camera_query.into_inner();
        camera
            .viewport_to_world_2d(camera_transform, pos)
            .unwrap_or(vec2(0.0, 0.0))
    });

    let player_pos = player_transform.translation().truncate();
    let aim_direction_vec = mouse_coords.unwrap_or_default() - player_pos;

    if aim_direction_vec != Vec2::ZERO {
        let new_direction = aim_direction_vec.y.atan2(aim_direction_vec.x);
        aim_direction.0 = new_direction;
        aim_direction_resource.0 = new_direction;
    }
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, (With<Player>, With<ControlledPlayer>)>,
) {
    let mut intent = Vec2::ZERO;
    if input.any_pressed(UP) {
        intent.y += 1.0;
    }
    if input.any_pressed(DOWN) {
        intent.y -= 1.0;
    }
    if input.any_pressed(LEFT) {
        intent.x -= 1.0;
    }
    if input.any_pressed(RIGHT) {
        intent.x += 1.0;
    }
    let intent = intent.normalize_or_zero();

    // Appliquer uniquement au joueur contrôlé localement
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

/// Ressource contenant les assets du joueur.
///
/// Charge et stocke les images, sons et autres ressources utilisées par le joueur.
/// Cette ressource est automatiquement chargée au démarrage du plugin.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,

    /// Liste des sons de pas utilisés pour le déplacement du joueur.
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
