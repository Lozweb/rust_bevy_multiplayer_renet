# Architecture Serveur

## Structure des Dossiers

```
server/src/
├── main.rs              // Point d'entrée, App Bevy
├── server.rs            // Plugin principal serveur
├── config.rs            // Configuration serveur (port, etc.)
├── resource/
│   ├── mod.rs
│   └── server_lobby.rs  // Gestion joueurs connectés
└── system/
    ├── mod.rs
    ├── server_event.rs  // Gestion connexions/déconnexions
    ├── input_handler.rs // Réception inputs clients
    ├── position_sync.rs // Broadcast positions @ 30Hz
    └── level.rs         // Spawn arène avec physique
```

## Responsabilités

### Autorité Totale (Full Server Authority)

- **Seule source de vérité** pour état du jeu
- **Physique complète** : Avian2D (RigidBody, Collider, LinearVelocity)
- **Calcul mouvement** : forces appliquées via physique
- **Détection collisions** : gérée par moteur physique
- **Validation** : toutes actions joueurs validées côté serveur

### Gestion Réseau

- **Réception inputs** : canal reliable depuis clients
- **Broadcast positions** : canal unreliable, 30Hz vers TOUS
- **Gestion connexions** : spawn/despawn joueurs automatique
- **Synchronisation** : état initial envoyé aux nouveaux joueurs

## Composants Joueur (Serveur)

### Physique

```rust
RigidBody::Dynamic              // Physique active
Collider::capsule(...)          // Forme collision
Mass::new(50.0)                 // Masse
LinearVelocity::ZERO            // Vélocité
Friction::new(0.1)              // Friction
Restitution::new(0.0)           // Rebond (0 = pas de rebond)
LinearDamping(1.5)              // Amortissement (réactivité)
```

### Gameplay

```rust
MovementController              // Vitesse max, accélération
PlayerInput                     // Inputs reçus du client
AimDirection                    // Direction visée
PlayerInfo                      // id, nom
```

### Visuel (même structure que client)

```rust
Sprite, Mesh2d, MeshMaterial2d, Transform
```

## Systèmes Principaux

### `server_event_system`

**Rôle** : Gestion connexions/déconnexions

```rust
// Nouvelle connexion
if event == ServerEvent::ClientConnected { id, ... }
    spawn_player(id)
    send PlayerConnected à TOUS
    send NetworkedEntities (état complet) au nouveau

// Déconnexion
if event == ServerEvent::ClientDisconnected { id, ... }
    despawn joueur
    send PlayerDisconnected à TOUS
```

### `process_client_inputs`

**Rôle** : Réception et application inputs clients

```rust
// Pour chaque client
while let Some(message) = server.receive_message(client_id, DefaultChannel)
    if let ClientMessage::Move(input, aim) = deserialize(message)
        // Trouver entité joueur
        Query<(&PlayerInfo, &mut PlayerInput, &mut AimDirection)>
        // Mettre à jour
        *player_input = input;
        *aim_direction = aim;
```

**Fréquence** : variable (reçoit quand client envoie)

### `interpolate_movement_intent`

**Rôle** : Lissage changements de direction

```rust
Query<(&PlayerInput, &mut MovementController)>

// Calcule direction cible
let target_direction = Vec2::new(
    if input.right { 1.0 } else if input.left { -1.0 } else { 0.0 },
    if input.up { 1.0 } else if input.down { -1.0 } else { 0.0 }
).normalize_or_zero();

// Interpole vers cible (évite changements brutaux)
controller.current_direction = controller.current_direction.lerp(target_direction, 0.1);
```

**Pourquoi ?** Évite à-coups quand joueur change brusquement de direction

### `apply_movement`

**Rôle** : Application forces physiques

```rust
Query<(&MovementController, &mut LinearVelocity)>

// Calcule vélocité cible
let target_velocity = controller.current_direction * controller.max_speed;

// Interpole vélocité actuelle vers cible
velocity.0 = velocity.0.lerp(target_velocity, controller.acceleration * delta_time);
```

**Fréquence** : 60 FPS (FixedUpdate)

**Pourquoi lerp ?**

- Mouvement fluide, pas instantané
- Les collisions physiques fonctionnent correctement
- Pas d'écrasement brutal de LinearVelocity

### `broadcast_player_positions`

**Rôle** : Envoi positions à tous les clients

```rust
// Timer @ 30Hz (0.033s)
timer.tick(delta);
if !timer.finished() { return; }

// Collecte toutes positions
Query<(&PlayerInfo, &Transform, &AimDirection)>
let entities: Vec<NetworkedEntity> = query.iter()
    .map(|(info, transform, aim)| NetworkedEntity {
        id: info.id,
        position: transform.translation.truncate(),
        aim_direction: aim.angle,
    })
    .collect();

// Broadcast unreliable
let msg = ServerMessages::NetworkedEntities(entities);
server.broadcast_message(NetworkedEntities::id(), serialize(&msg));
```

**Fréquence** : 30Hz (optimisation bande passante)
**Canal** : Unreliable (perte acceptable, données temporelles)

## Paramètres Physique

### Mouvement

```rust
ACCELERATION: 30.0          // Vitesse d'accélération
MAX_SPEED: 200.0            // Vitesse maximale
LINEAR_DAMPING: 1.5         // Amortissement (réactivité)
```

### Collisions

```rust
FRICTION: 0.1               // Friction entre objets
RESTITUTION: 0.0            // Rebond (0 = aucun)
MASS: 50.0                  // Masse joueur
```

### Réseau

```rust
BROADCAST_RATE: 30Hz        // Fréquence envoi positions
TICK_RATE: 60 FPS           // Physique serveur
```

## Flux de Données

```
[Client 1, 2, 3...] 
    ↓ ClientMessage::Move (variable, si changement)
server_receive_messages
    ↓
process_client_inputs
    ↓
PlayerInput, AimDirection (Components)
    ↓
interpolate_movement_intent @ 60 FPS
    ↓
MovementController.current_direction
    ↓
apply_movement @ 60 FPS
    ↓
LinearVelocity (Component)
    ↓
Avian2D Physics Step @ 60 FPS
    ↓
Transform (position finale)
    ↓
broadcast_player_positions @ 30 Hz
    ↓
ServerMessages::NetworkedEntities
    ↓
[Client 1, 2, 3...] ← Unreliable
```

## Gestion Level

### `spawn_level`

```rust
// Murs avec physique
commands.spawn((
    Mesh2d(...),
    MeshMaterial2d(...),
    Transform::from_xyz(x, y, 0.0),
    RigidBody::Static,          // Obstacle statique
    Collider::rectangle(w, h),  // Forme collision
));
```

**Important** : TOUS les obstacles doivent avoir physique serveur

## Configuration Serveur

### `ServerConfig`

```rust
pub struct ServerConfig {
    pub port: u16,              // Port écoute (5000)
    pub max_clients: usize,     // Limite joueurs (64)
    pub protocol_id: u64,       // ID protocole Renet
}
```

### `ServerLobby`

```rust
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,  // client_id -> entity
}
```

- Suivi joueurs connectés
- Mapping ID réseau ↔ Entité Bevy

## Ordre Exécution Systèmes

```rust
app.add_systems(Update, (
    server_event_system,        // 1. Connexions/déconnexions
    process_client_inputs,      // 2. Réception inputs
));

app.add_systems(FixedUpdate, (
    interpolate_movement_intent, // 1. Lissage direction
    apply_movement,              // 2. Application forces
    // Avian2D physics step automatique
).chain());

app.add_systems(Update, (
    broadcast_player_positions,  // Après physics
));
```

**Pourquoi FixedUpdate ?**

- Physique déterministe (60 FPS fixe)
- Pas de variations selon framerate
- Cohérence entre serveurs différents

## Optimisations

### Bande Passante

- Broadcast 30Hz au lieu de 60Hz (-50% données)
- Canal unreliable pour positions (perte acceptable)
- Seuil changement visée client (0.01 rad)

### Performance

- Interpolation direction (évite calculs physiques complexes)
- Lerp vélocité (pas de forces brutales)
- Timer précis pour broadcast

## Pièges à Éviter

❌ **NE JAMAIS** écraser brutalement LinearVelocity
→ `velocity.0 = target` → collisions cassées
→ Utiliser `velocity.0 = velocity.0.lerp(target, t)`

❌ **NE JAMAIS** oublier physique sur obstacles
→ Joueurs traverseront les murs

❌ **NE JAMAIS** broadcast à 60Hz
→ Bande passante excessive, utiliser 30Hz

❌ **NE JAMAIS** utiliser canal Reliable pour positions
→ Latence accrue, utiliser Unreliable

❌ **NE JAMAIS** oublier de broadcast état initial aux nouveaux clients
→ Ils verront un monde vide

✅ **TOUJOURS** utiliser interpolation pour vélocité
✅ **TOUJOURS** RigidBody::Static sur obstacles
✅ **TOUJOURS** broadcast avec timer précis
✅ **TOUJOURS** envoyer état complet aux nouveaux connectés

