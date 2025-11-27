use crate::enemy::{spawn_enemy, EnemyServerEntity, EnemyType};
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct ServerLevel;

pub const LEVEL_ARENA_WIDTH: f32 = 1200.0;
pub const LEVEL_ARENA_HEIGHT: f32 = 800.0;
pub const LEVEL_WALL_THICKNESS: f32 = 20.0;

pub fn setup_level(mut commands: Commands) {
    commands
        .spawn((
            Name::new("ServerLevel"),
            ServerLevel,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn(wall_bundle(
                "WallTop".to_string(),
                Vec3::new(0.0, LEVEL_ARENA_HEIGHT / 2.0, 0.0),
                Vec2::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS),
            ));

            parent.spawn(wall_bundle(
                "WallBottom".to_string(),
                Vec3::new(0.0, -LEVEL_ARENA_HEIGHT / 2.0, 0.0),
                Vec2::new(LEVEL_ARENA_HEIGHT, LEVEL_WALL_THICKNESS),
            ));

            parent.spawn(wall_bundle(
                "WallLeft".to_string(),
                Vec3::new(-LEVEL_ARENA_HEIGHT / 2.0, 0.0, 0.0),
                Vec2::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT),
            ));

            parent.spawn(wall_bundle(
                "WallRight".to_string(),
                Vec3::new(LEVEL_ARENA_HEIGHT / 2.0, 0.0, 0.0),
                Vec2::new(LEVEL_WALL_THICKNESS, LEVEL_ARENA_HEIGHT),
            ));

            parent.spawn(wall_bundle(
                "CentralObstacle".to_string(),
                Vec3::ZERO,
                Vec2::new(100.0, 100.0),
            ));

            parent.spawn(wall_bundle(
                "ObstacleLeft".to_string(),
                Vec3::new(-300.0, 100.0, 0.0),
                Vec2::new(80.0, 80.0),
            ));

            parent.spawn(wall_bundle(
                "ObstacleRight".to_string(),
                Vec3::new(300.0, 100.0, 0.0),
                Vec2::new(80.0, 80.0),
            ));
        });

    info!("Server level created");
}

pub fn wall_bundle(name: String, position: Vec3, size: Vec2) -> impl Bundle {
    (
        Name::new(name),
        Wall,
        Transform::from_translation(position),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    )
}

pub fn spawn_initial_enemies(mut commands: Commands) {
    let e1 = spawn_enemy(
        &mut commands,
        Vec3::new(200., 0., 0.),
        EnemyType::Basic,
        &mut None,
        &mut None,
    );
    commands
        .entity(e1)
        .insert(EnemyServerEntity { server_entity: e1 });

    let e2 = spawn_enemy(
        &mut commands,
        Vec3::new(300., 0., 0.),
        EnemyType::Medium,
        &mut None,
        &mut None,
    );
    commands
        .entity(e2)
        .insert(EnemyServerEntity { server_entity: e2 });

    let e3 = spawn_enemy(
        &mut commands,
        Vec3::new(350., 0., 0.),
        EnemyType::Hard,
        &mut None,
        &mut None,
    );
    commands
        .entity(e3)
        .insert(EnemyServerEntity { server_entity: e3 });
}
