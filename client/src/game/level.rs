use crate::audio::music;
use crate::screens::Screen;
use avian2d::prelude::*;
use bevy::prelude::*;
use game_core::asset_tracking::LoadResource;

#[derive(Component)]
pub struct Level;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/space_ambiance.ogg"),
        }
    }
}

/// Système qui spawn le niveau principal.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.clone()),
            ));

            // Ajouter les murs visuels (correspond au serveur)
            // Note: Pas de physique côté client, seulement visuel
            let wall_material = materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.4)));
            let obstacle_material = materials.add(ColorMaterial::from(Color::srgb(0.5, 0.3, 0.3)));

            // Dimensions de l'arène (identiques au serveur)
            let arena_width = 1200.0;
            let arena_height = 800.0;
            let wall_thickness = 20.0;

            // Mur du haut
            parent.spawn((
                Name::new("WallTop"),
                Mesh2d(meshes.add(Rectangle::new(arena_width, wall_thickness))),
                MeshMaterial2d(wall_material.clone()),
                Transform::from_xyz(0.0, arena_height / 2.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            // Mur du bas
            parent.spawn((
                Name::new("WallBottom"),
                Mesh2d(meshes.add(Rectangle::new(arena_width, wall_thickness))),
                MeshMaterial2d(wall_material.clone()),
                Transform::from_xyz(0.0, -arena_height / 2.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            // Mur de gauche
            parent.spawn((
                Name::new("WallLeft"),
                Mesh2d(meshes.add(Rectangle::new(wall_thickness, arena_height))),
                MeshMaterial2d(wall_material.clone()),
                Transform::from_xyz(-arena_width / 2.0, 0.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            // Mur de droite
            parent.spawn((
                Name::new("WallRight"),
                Mesh2d(meshes.add(Rectangle::new(wall_thickness, arena_height))),
                MeshMaterial2d(wall_material.clone()),
                Transform::from_xyz(arena_width / 2.0, 0.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            // Obstacle central
            parent.spawn((
                Name::new("CentralObstacle"),
                Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
                MeshMaterial2d(obstacle_material.clone()),
                Transform::from_xyz(0.0, 0.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(100.0, 100.0),
            ));

            // Obstacles supplémentaires
            parent.spawn((
                Name::new("ObstacleLeft"),
                Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
                MeshMaterial2d(obstacle_material.clone()),
                Transform::from_xyz(-300.0, 100.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(80.0, 80.0),
            ));

            parent.spawn((
                Name::new("ObstacleRight"),
                Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
                MeshMaterial2d(obstacle_material),
                Transform::from_xyz(300.0, -100.0, -0.5),
                RigidBody::Static,
                Collider::rectangle(80.0, 80.0),
            ));
        });
}
