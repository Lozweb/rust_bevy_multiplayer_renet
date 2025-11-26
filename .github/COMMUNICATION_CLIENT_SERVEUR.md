# Schéma de Communication Client ↔ Serveur

## Vue d'Ensemble

```
Client                                    Serveur
  |                                          |
  |========== CONNEXION ====================>|
  |<========= ÉTAT INITIAL ==================|
  |                                          |
  |========== INPUTS (variable) ============>|
  |<========= POSITIONS (30Hz) ==============|
  |                                          |
  |========== DÉCONNEXION ==================>|
```

## Phase 1 : Connexion

### Étape par Étape

```
Client                                    Serveur
  |                                          |
  | 1. Démarrage client                      |
  | create RenetClient                       |
  | connect to 127.0.0.1:5000                |
  |                                          |
  |-- ConnectionRequest ------------------> |
  |   (Renet transport layer)                | 2. ServerEvent::ClientConnected
  |                                          |    { id: u64, user_data: ... }
  |                                          |
  |                                          | 3. spawn_player(id)
  |                                          |    PlayerBundle {
  |                                          |      RigidBody::Dynamic,
  |                                          |      Collider, Mass, etc.
  |                                          |    }
  |                                          |
  |                                          | 4. Broadcast à TOUS
  | <-- PlayerConnected(id, "Player N") -----| 
  |     (DefaultChannel - Reliable)          |
  |                                          |
  | 5. spawn_player(id, name, pos)           |
  |    SI id != local : mark as networked    |
  |                                          |
  | <-- NetworkedEntities(Vec<...>) ---------| 6. Envoi état complet au nouveau
  |     (Unreliable)                         |    Tous joueurs existants
  |                                          |
  | 7. spawn_player pour chaque entité       |
  |    mark all as networked (pas local)     |
```

### Messages Échangés

#### Serveur → Tous les Clients

```rust
ServerMessages::PlayerConnected(u64, String)
// Canal: DefaultChannel (Reliable/Ordered)
// Contenu: { id: 12345, name: "Player 3" }
```

#### Serveur → Nouveau Client Uniquement

```rust
ServerMessages::NetworkedEntities(Vec<NetworkedEntity>)
// Canal: NetworkedEntities (Unreliable)
// Contenu: [
//   { id: 1, position: Vec2(100, 200), aim_direction: 1.57 },
//   { id: 2, position: Vec2(150, 300), aim_direction: 0.0 },
// ]
```

## Phase 2 : Boucle Gameplay

### Timeline (60 FPS client, 60 FPS serveur, 30 FPS broadcast)

```
Temps   Client                                 Serveur
--------------------------------------------------------------------
0ms     [Frame 1 @ 60 FPS]                     [Frame 1 @ 60 FPS]
        record_player_directional_input        
        - Keyboard: W=true, A=false...         
        - Mouse: world_pos → aim_angle         
        - Update PlayerInput (Resource)        
        - Update AimDirection (Resource)       
                                                
        input_sync_system                      
        - Compare previous_input vs current    
        - IF different:                        
          |-- Move(input, aim) -------------->  process_client_inputs
          |   (DefaultChannel - Reliable)         - Find player by id
                                                   - Update PlayerInput
                                                   - Update AimDirection
                                                
                                                interpolate_movement_intent
                                                - Lerp current_direction → target
                                                - Smooth direction changes
                                                
                                                apply_movement
                                                - target_vel = dir * max_speed
                                                - vel.lerp(target_vel, accel*dt)
                                                
                                                [Avian2D Physics Step]
                                                - Apply forces
                                                - Resolve collisions
                                                - Update Transform
                                                
16ms    [Frame 2 @ 60 FPS]                     [Frame 2 @ 60 FPS]
        ...                                     ...
                                                
33ms    [Frame 3 @ 60 FPS]                     [Frame 3 @ 60 FPS]
                                                
                                                broadcast_player_positions
                                                - timer reached 0.033s
                                                - Collect all positions
                                                |-- NetworkedEntities(Vec) -----> 
                                                    (Unreliable @ 30Hz)
                                                
        receive_position_updates               
        <-- NetworkedEntities ------------------|
        - For each NetworkedEntity:            
          - Find player by id                  
          - Update NetworkedTransform.target   
                                                
        reconcile_local_player (si local)      
        - distance = server_pos - local_pos    
        - IF distance > 15.0:                  
          - local_pos.lerp(server_pos, 3.0*dt) 
                                                
        interpolate_networked_players          
        - Query: Without<ControlledPlayer>     
        - transform.lerp(target, 25.0 * dt)    
                                                
50ms    [Frame 4 @ 60 FPS]                     [Frame 4 @ 60 FPS]
        ...                                     ...
```

### Fréquences

| Action             | Client                   | Serveur              |
|--------------------|--------------------------|----------------------|
| Frame Rate         | 60 FPS                   | 60 FPS               |
| Input Capture      | 60 FPS                   | -                    |
| Input Send         | Variable (si changement) | -                    |
| Input Receive      | -                        | Variable             |
| Physics Step       | -                        | 60 FPS (FixedUpdate) |
| Position Broadcast | -                        | 30 Hz (0.033s)       |
| Position Receive   | 30 Hz                    | -                    |
| Interpolation      | 60 FPS                   | -                    |

### Optimisation Envoi Inputs

```rust
// Client: input_sync_system
fn input_sync_system(
    input: Res<PlayerInput>,
    aim: Res<AimDirection>,
    mut previous: Local<Option<(PlayerInput, AimDirection)>>,
    mut client: ResMut<RenetClient>,
) {
    let current = (*input, *aim);
    
    // Vérification changement
    if let Some(prev) = *previous {
        // Seuil pour visée (0.01 rad ≈ 0.57°)
        let aim_changed = (current.1.angle - prev.1.angle).abs() > 0.01;
        let input_changed = current.0 != prev.0;
        
        if !aim_changed && !input_changed {
            return; // Pas d'envoi si aucun changement
        }
    }
    
    // Envoi seulement si changement détecté
    let msg = ClientMessage::Move(current.0, current.1);
    client.send_message(DefaultChannel::id(), bincode::serialize(&msg).unwrap());
    
    *previous = Some(current);
}
```

**Réduction bande passante** : ~90% (60 msg/s → 5-10 msg/s)

## Phase 3 : Déconnexion

### Déconnexion Normale

```
Client                                    Serveur
  |                                          |
  | 1. User ferme fenêtre                    |
  | drop(RenetClient)                        |
  |                                          |
  |-- DisconnectionRequest ---------------> |
  |   (Renet transport)                      | 2. ServerEvent::ClientDisconnected
  |                                          |    { id: u64, reason: ... }
  X                                          |
                                             | 3. Query player by id
                                             | commands.entity(player).despawn()
                                             |
                                             | 4. Broadcast à tous
                                             |-- PlayerDisconnected(id) --------> [Autres Clients]
                                             |   (DefaultChannel - Reliable)
                                             
                                             [Autres Clients]
                                             5. Query player by id
                                             despawn entity
```

### Déconnexion Timeout

```
Client                                    Serveur
  |                                          |
  | [Perte connexion réseau]                 |
  X                                          | [Pas de heartbeat pendant N sec]
                                             |
                                             | ServerEvent::ClientDisconnected
                                             | { id, reason: Timeout }
                                             |
                                             | despawn player
                                             | broadcast PlayerDisconnected(id)
```

## Canaux de Communication

### DefaultChannel (Reliable/Ordered)

**Utilisation**

- Messages critiques nécessitant garantie de livraison
- Ordre des messages important

**Messages**

```rust
// Client → Serveur
ClientMessage::Move(PlayerInput, AimDirection)

// Serveur → Client
ServerMessages::PlayerConnected(u64, String)
ServerMessages::PlayerDisconnected(u64)
```

**Caractéristiques**

- ✅ Garantie livraison (re-transmission si perte)
- ✅ Ordre préservé (FIFO)
- ❌ Latence plus élevée si perte paquets
- ❌ Bande passante supérieure (acknowledge)

### NetworkedEntities (Unreliable)

**Utilisation**

- Données temporelles (positions)
- Mise à jour fréquente (30Hz)
- Perte acceptable

**Messages**

```rust
// Serveur → Client
ServerMessages::NetworkedEntities(Vec<NetworkedEntity>)

pub struct NetworkedEntity {
    pub id: u64,
    pub position: Vec2,
    pub aim_direction: f32,
}
```

**Caractéristiques**

- ✅ Latence minimale (pas de re-transmission)
- ✅ Bande passante optimale (pas d'acknowledge)
- ❌ Pas de garantie livraison (perte possible)
- ❌ Ordre non garanti

**Pourquoi Unreliable pour positions ?**

- Données périmées si retard → paquet récent suffit
- Fréquence élevée (30Hz) → perte compensée par prochain
- Latence critique pour gameplay fluide

## Configuration Renet

### Client

```rust
let client = RenetClient::new(ConnectionConfig {
    protocol_id: 0x12345, // Doit matcher serveur
    client_authentication: ClientAuthentication::Unsecure {
        server_addr: "127.0.0.1:5000".parse().unwrap(),
        client_id: generate_random_u64(),
        user_data: None,
    },
    ..Default::default()
});

// Canaux
let connection_config = ConnectionConfig::default();
connection_config.channels = vec![
    ChannelConfig::default(), // DefaultChannel (Reliable)
    ChannelConfig {
        channel_id: 1,
        max_memory_usage_bytes: 5 * 1024 * 1024, // 5MB
        send_type: SendType::Unreliable,
    }, // NetworkedEntities
];
```

### Serveur

```rust
let server = RenetServer::new(ConnectionConfig {
    protocol_id: 0x12345, // Doit matcher client
    max_clients: 64,
    ..Default::default()
});

// Même configuration canaux
```

## Débit Réseau

### Client → Serveur (Montant)

**Sans optimisation** : 60 msg/s × 32 bytes = 1.92 KB/s
**Avec optimisation** : 5-10 msg/s × 32 bytes = 0.16-0.32 KB/s

**Réduction** : ~90%

### Serveur → Client (Descendant)

**Par joueur** : 30 Hz × (8 + 8 + 4) bytes = 600 bytes/s
**Pour 10 joueurs** : 600 × 10 = 6 KB/s

**Total bidirectionnel** : ~6.5 KB/s par client (avec 10 joueurs)

## Diagramme Complet

```
┌─────────────────────────────────────────────────────────────────┐
│                          CONNEXION                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Client                              Serveur                   │
│    │                                    │                       │
│    │──── ConnectionRequest ────────────>│                       │
│    │                                    │ spawn_player          │
│    │<─── PlayerConnected ───────────────│                       │
│    │<─── NetworkedEntities ─────────────│ (état initial)        │
│    │                                    │                       │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                       GAMEPLAY LOOP                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Client @ 60 FPS                     Serveur @ 60 FPS           │
│    │                                    │                       │
│    │ Capture Inputs                     │                       │
│    │ (clavier + souris)                 │                       │
│    │                                    │                       │
│    │──── ClientMessage::Move ──────────>│ (variable, si Δ)      │
│    │     (Reliable)                     │                       │
│    │                                    │ Store PlayerInput     │
│    │                                    │ Store AimDirection    │
│    │                                    │                       │
│    │                                    │ Interpolate Intent    │
│    │                                    │ Apply Movement        │
│    │                                    │ Physics Step          │
│    │                                    │                       │
│    │                                    │ Broadcast @ 30Hz      │
│    │<─── NetworkedEntities ─────────────│                       │
│    │     (Unreliable)                   │                       │
│    │                                    │                       │
│    │ Update Targets                     │                       │
│    │ Reconcile Local (si divergence)    │                       │
│    │ Interpolate Distants               │                       │
│    │                                    │                       │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                       DÉCONNEXION                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Client                              Serveur                   │
│    │                                    │                       │
│    │──── DisconnectionRequest ─────────>│                       │
│    X                                    │ despawn_player        │
│                                         │                       │
│                                         │──> Broadcast          │
│                                         │    PlayerDisconnected │
│                                         │                       │
└─────────────────────────────────────────────────────────────────┘
```

## Features Actuellement Implémentées

### ✅ Connexion/Déconnexion

- Gestion automatique joueurs
- Synchronisation état initial
- Broadcast events connexion

### ✅ Mouvement Multijoueur

- Joueur local : physique + réconciliation douce
- Joueurs distants : interpolation pure
- Collisions entre joueurs (poussée)
- Obstacles statiques

### ✅ Synchronisation Visée

- Direction souris envoyée au serveur
- Interpolation pour joueurs distants
- Seuil anti-spam (0.01 rad)

### ✅ Optimisations Réseau

- Envoi conditionnel inputs (-90% bande passante)
- Broadcast 30Hz (pas 60Hz)
- Canaux adaptés (reliable/unreliable)
- Réconciliation douce (seuil 15.0)

## Latence et Compensation

### Latence Typique

- LAN : 1-5ms
- Internet proche : 20-50ms
- Internet moyen : 50-100ms

### Stratégies Implémentées

#### Joueur Local

- **Physique locale immédiate** : réponse instantanée
- **Réconciliation douce** : correction si divergence > 15.0
- **Pas de prédiction complexe** : simplicité > complexité

#### Joueurs Distants

- **Interpolation** : lissage vers positions serveur
- **Vitesse adaptée** : 25.0 pour compenser 30Hz + latence
- **Accepte perte paquets** : canal unreliable

### Feeling Joueur

- **Latence perçue locale** : 0ms (physique immédiate)
- **Latence perçue distants** : 50-100ms (acceptable)
- **Compromis** : Full Server Authority = simple + fiable

## Pièges à Éviter

❌ **NE JAMAIS** utiliser canal Reliable pour positions
→ Latence accrue si perte paquets

❌ **NE JAMAIS** broadcast à 60Hz
→ Bande passante excessive

❌ **NE JAMAIS** envoyer inputs sans vérifier changement
→ Spam réseau inutile

❌ **NE JAMAIS** oublier d'envoyer état initial aux nouveaux
→ Monde vide pour le nouveau joueur

✅ **TOUJOURS** utiliser Unreliable pour données temporelles
✅ **TOUJOURS** vérifier changement avant envoi
✅ **TOUJOURS** envoyer état complet à la connexion
✅ **TOUJOURS** gérer déconnexions proprement

