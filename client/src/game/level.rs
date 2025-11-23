use crate::audio::music;
use crate::screens::Screen;
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
    _materials: ResMut<Assets<ColorMaterial>>,
    _meshes: ResMut<Assets<Mesh>>,
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
            // TODO: Charger le level depuis le serveur
            parent.spawn((
                Name::new("Gameplay Music"),
                music(level_assets.music.clone()),
            ));
        });
}
