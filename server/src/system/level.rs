/// Système de gestion du level côté serveur
///
/// Crée l'environnement de jeu avec des murs et obstacles que les joueurs ne peuvent pas traverser.
use avian2d::prelude::*;
use bevy::prelude::*;


/// Marqueur pour identifier les murs du level
#[derive(Component)]
pub struct Wall;

/// Marqueur pour identifier l'entité contenant le level
#[derive(Component)]
pub struct ServerLevel;

/// Crée le level initial avec des murs de bordure
pub fn setup_level(mut commands: Commands) {
    info!("Setting up server level with walls");

    // Entité parent pour le level
    commands
        .spawn((
            Name::new("ServerLevel"),
            ServerLevel,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Dimensions de l'arène
            let arena_width = 1200.0;
            let arena_height = 800.0;
            let wall_thickness = 20.0;

            // Mur du haut
            parent.spawn((
                Name::new("WallTop"),
                Wall,
                Transform::from_xyz(0.0, arena_height / 2.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            // Mur du bas
            parent.spawn((
                Name::new("WallBottom"),
                Wall,
                Transform::from_xyz(0.0, -arena_height / 2.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(arena_width, wall_thickness),
            ));

            // Mur de gauche
            parent.spawn((
                Name::new("WallLeft"),
                Wall,
                Transform::from_xyz(-arena_width / 2.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            // Mur de droite
            parent.spawn((
                Name::new("WallRight"),
                Wall,
                Transform::from_xyz(arena_width / 2.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(wall_thickness, arena_height),
            ));

            // Obstacle central pour tester les collisions
            parent.spawn((
                Name::new("CentralObstacle"),
                Wall,
                Transform::from_xyz(0.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(100.0, 100.0),
            ));

            // Quelques obstacles supplémentaires
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

    info!("Server level created with walls and obstacles");
}
