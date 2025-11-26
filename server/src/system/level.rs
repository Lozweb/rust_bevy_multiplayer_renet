use avian2d::prelude::*;
use bevy::prelude::*;
use game_core::enemy::{spawn_enemy, EnemyServerEntity, EnemyType};

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct ServerLevel;

pub fn setup_level(mut commands: Commands) {
    commands
        .spawn((
            Name::new("ServerLevel"),
            ServerLevel,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            let arena_width = 1200.0;
            let arena_height = 800.0;
            let wall_thickness = 20.0;

            parent.spawn((
                Name::new("WallTop"),
                Wall,
                Transform::from_xyz(0.0, arena_height / 2.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            parent.spawn((
                Name::new("WallBottom"),
                Wall,
                Transform::from_xyz(0.0, -arena_height / 2.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            parent.spawn((
                Name::new("WallLeft"),
                Wall,
                Transform::from_xyz(-arena_width / 2.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            parent.spawn((
                Name::new("WallRight"),
                Wall,
                Transform::from_xyz(arena_width / 2.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            parent.spawn((
                Name::new("CentralObstacle"),
                Wall,
                Transform::from_xyz(0.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(100.0, 100.0),
            ));

            parent.spawn((
                Name::new("ObstacleLeft"),
                Wall,
                Transform::from_xyz(-300.0, 100.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(80.0, 80.0),
            ));

            parent.spawn((
                Name::new("ObstacleRight"),
                Wall,
                Transform::from_xyz(300.0, -100.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(80.0, 80.0),
            ));
        });

    info!("Server level created");
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
