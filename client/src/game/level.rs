use crate::audio::music;
use crate::screens::Screen;
use bevy::prelude::*;
use game_core::asset_tracking::LoadResource;
use game_core::level::{wall_bundle, LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS};

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

            let wall_material = materials.add(ColorMaterial::from(Color::srgb(0.3, 0.3, 0.4)));
            let obstacle_material = materials.add(ColorMaterial::from(Color::srgb(0.5, 0.3, 0.3)));

            parent
                .spawn(wall_bundle(
                    "WallTop".to_string(),
                    Vec3::new(0.0, LEVEL_ARENA_HEIGHT / 2.0, 0.0),
                    Vec2::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS))),
                    MeshMaterial2d(wall_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "WallBottom".to_string(),
                    Vec3::new(0.0, -LEVEL_ARENA_HEIGHT / 2.0, 0.0),
                    Vec2::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS))),
                    MeshMaterial2d(wall_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "WallLeft".to_string(),
                    Vec3::new(-LEVEL_ARENA_HEIGHT / 2.0, 0.0, 0.0),
                    Vec2::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT))),
                    MeshMaterial2d(wall_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "WallRight".to_string(),
                    Vec3::new(LEVEL_ARENA_HEIGHT / 2.0, 0.0, 0.0),
                    Vec2::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT))),
                    MeshMaterial2d(wall_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "CentralObstacle".to_string(),
                    Vec3::ZERO,
                    Vec2::new(100.0, 100.0),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
                    MeshMaterial2d(obstacle_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "ObstacleLeft".to_string(),
                    Vec3::new(-300.0, 100.0, 0.0),
                    Vec2::new(80.0, 80.0),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
                    MeshMaterial2d(obstacle_material.clone()),
                ));

            parent
                .spawn(wall_bundle(
                    "ObstacleRight".to_string(),
                    Vec3::new(300.0, 100.0, 0.0),
                    Vec2::new(80.0, 80.0),
                ))
                .insert((
                    Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
                    MeshMaterial2d(obstacle_material),
                ));
        });
}
