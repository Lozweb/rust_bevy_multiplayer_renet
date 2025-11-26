# Architecture Game Core

## Structure

```
game_core/src/
├── lib.rs              // Exports publics, registration types
├── player.rs           // PlayerBundle, spawn_player(), Components
├── network.rs          // Messages réseau (ClientMessage, ServerMessages)
├── client.rs           // Types spécifiques client (si existe)
├── server.rs           // Types spécifiques serveur (si existe)
└── asset_tracking.rs   // Gestion assets partagés (si existe)
```

## Rôle du Game Core

**Code partagé entre client et serveur** pour garantir cohérence et éviter duplication.

### Principe

- Une seule définition de chaque structure
- Pas de désynchronisation client/serveur
- Types serialisables pour réseau
- Reflect pour inspection runtime

## Components Partagés

### `PlayerInput`

```rust
#[derive(Component, Resource, Reflect, Serialize, Deserialize, Clone, Copy)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
```

- Utilisé comme **Component** sur entités joueur
- Utilisé comme **Resource** pour stockage temporaire inputs
- Serialisé pour envoi réseau

### `AimDirection`

```rust
#[derive(Component, Resource, Reflect, Serialize, Deserialize, Clone, Copy)]
pub struct AimDirection {
    pub angle: f32,
}
```

- Direction de visée en radians
- Partagé client/serveur pour cohérence
- `#[reflect(Component, Resource)]` pour flexibilité

### `MovementController`

```rust
#[derive(Component, Reflect)]
pub struct MovementController {
    pub max_speed: f32,
    pub acceleration: f32,
}
```

- Paramètres mouvement identiques client/serveur
- Valeurs typiques : max_speed: 200.0, acceleration: 30.0

### `PlayerInfo`

```rust
#[derive(Component)]
pub struct PlayerInfo {
    pub id: u64,
    pub name: String,
}
```

- Identifiant unique réseau
- Nom du joueur

### `ControlledPlayer`

```rust
#[derive(Component)]
pub struct ControlledPlayer;
```

- Marqueur pour le joueur contrôlé localement
- Utilisé par queries pour filtrer (With/Without)

## Messages Réseau

### `ClientMessage`

```rust
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Move(PlayerInput, AimDirection),
}
```

- Messages envoyés du **client vers serveur**
- Canal : DefaultChannel (Reliable/Ordered)
- Move : inputs clavier + direction visée

### `ServerMessages`

```rust
#[derive(Serialize, Deserialize)]
pub enum ServerMessages {
    PlayerConnected(u64, String),
    PlayerDisconnected(u64),
    NetworkedEntities(Vec<NetworkedEntity>),
}
```

- Messages envoyés du **serveur vers clients**
- PlayerConnected/Disconnected : canal Reliable
- NetworkedEntities : canal Unreliable @ 30Hz

### `NetworkedEntity`

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkedEntity {
    pub id: u64,
    pub position: Vec2,
    pub aim_direction: f32,
}
```

- Snapshot position d'une entité
- Envoyé dans NetworkedEntities (broadcast 30Hz)

## Fonction de Spawn

### `spawn_player()`

```rust
pub fn spawn_player(
    commands: &mut Commands,
    id: u64,
    name: String,
    position: Vec2,
) -> Entity {
    commands.spawn(PlayerBundle {
        player_info: PlayerInfo { id, name },
        movement_controller: MovementController {
            max_speed: 200.0,
            acceleration: 30.0,
        },
        player_input: PlayerInput::default(),
        aim_direction: AimDirection { angle: 0.0 },
        // ... sprite, transform, etc.
    }).id()
}
```

**Utilisation :**

- **Serveur** : spawn avec physique complète (RigidBody, Collider, etc.)
- **Client** : spawn sans physique (juste visuel + Collider)
- Garantit structure de base identique

## PlayerBundle

```rust
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player_info: PlayerInfo,
    pub movement_controller: MovementController,
    pub player_input: PlayerInput,
    pub aim_direction: AimDirection,
    pub sprite: Sprite,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    // ... 16+ composants → nécessite #[derive(Bundle)]
}
```

**Pourquoi un Bundle ?**

- Bevy limite les tuples à 16 éléments
- Bundle permet de grouper 16+ Components
- Évite erreur E0277 "trait is not implemented"

## Registration de Types

```rust
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerInput>()
            .register_type::<AimDirection>()
            .register_type::<MovementController>()
            .register_type::<PlayerInfo>();
    }
}
```

**Pourquoi Register ?**

- Permet inspection avec Bevy Inspector
- Serialization/Deserialization scènes
- Debug runtime

## Utilisation dans Client/Serveur

### Client

```rust
use game_core::{
    PlayerInput, AimDirection, MovementController,
    ClientMessage, ServerMessages, spawn_player
};

// Envoi inputs
let msg = ClientMessage::Move(input, aim);
client.send_message(DefaultChannel::id(), bincode::serialize(&msg)?);

// Spawn joueur
spawn_player(&mut commands, id, name, position);
```

### Serveur

```rust
use game_core::{
    PlayerInput, AimDirection, MovementController,
    ClientMessage, ServerMessages, spawn_player
};

// Réception inputs
if let ClientMessage::Move(input, aim) = msg {
    // Appliquer au joueur
}

// Broadcast positions
let msg = ServerMessages::NetworkedEntities(entities);
server.broadcast_message(NetworkedEntities::id(), bincode::serialize(&msg)?);
```

## Avantages Architecture Partagée

✅ **Un seul endroit de définition** → pas de désynchronisation
✅ **Reflect automatique** → inspection runtime
✅ **Serde intégré** → serialization réseau
✅ **Types cohérents** → pas d'erreur de cast
✅ **Maintenance facile** → changement unique propagé partout

## Pièges à Éviter

❌ **NE JAMAIS** dupliquer structures dans client ET serveur
→ Utiliser game_core pour tout code partagé

❌ **NE JAMAIS** oublier `#[derive(Serialize, Deserialize)]` sur messages réseau
→ Erreur de compilation bincode

❌ **NE JAMAIS** dépasser 16 composants dans un tuple
→ Créer un `#[derive(Bundle)] struct` à la place

❌ **NE JAMAIS** oublier `register_type` pour Components Reflect
→ Inspection impossible

✅ **TOUJOURS** définir structures réseau dans game_core
✅ **TOUJOURS** utiliser Reflect pour Components
✅ **TOUJOURS** créer Bundle si > 16 composants

