# Architecture Client

## Structure des Dossiers

```
client/src/
├── main.rs              // Point d'entrée, App Bevy
├── screens/             // États UI (Title, Gameplay, etc.)
├── game/
│   ├── mod.rs          // Plugin principal
│   ├── player.rs       // Spawn joueur local + marquage distants
│   ├── movement.rs     // Capture inputs + synchronisation réseau
│   ├── level.rs        // Spawn arène (AVEC physique pour collisions)
│   └── camera.rs       // Caméra suivant le joueur
├── client/
│   ├── mod.rs          // Plugin réseau client
│   ├── input.rs        // Capture clavier/souris → PlayerInput
│   └── position_sync.rs // Réconciliation + Interpolation
└── network/            // (si existe - gestion Renet)
```

## Responsabilités

- **Affichage** : rendu visuel de tous les joueurs
- **Physique locale** : `RigidBody::Dynamic` pour le joueur local uniquement
- **Physique distante** : `RigidBody::Static` pour les autres (poussables)
- **Capture inputs** : clavier (WASD) + souris (visée)
- **Envoi inputs** : canal reliable, seulement si changement
- **Réception positions** : canal unreliable, 30Hz
- **Réconciliation douce** : correction position locale si divergence > 15.0
- **Interpolation** : lissage joueurs distants uniquement (pas le local)

## Composants Clés

```rust
ControlledPlayer         // Marqueur joueur local
NetworkedTransform       // Cible d'interpolation (position serveur)
RigidBody::Dynamic       // Physique active pour joueur local
RigidBody::Static        // Collision pour joueurs distants
Collider                 // Collision physique (tous)
PlayerInput              // Snapshot inputs (Component + Resource)
AimDirection             // Direction visée (Component + Resource)
MovementController       // Vitesse max, accélération
```

## Paramètres Critiques

```rust
LOCAL_RECONCILIATION_THRESHOLD: 15.0    // Divergence tolérée
LOCAL_RECONCILIATION_SPEED: 3.0         // Vitesse correction douce
INTERPOLATION_SPEED: 25.0               // Joueurs distants @ 30Hz
ACCELERATION: 30.0                      // Client et serveur
LINEAR_DAMPING: 1.5                     // Réactivité optimale
```

## Systèmes Principaux

### `record_player_directional_input`

- Capture les entrées clavier (WASD, ZQSD)
- Capture la position souris et calcule `AimDirection`
- Met à jour `PlayerInput` et `AimDirection` (Resource)

### `input_sync_system`

- Détecte les changements d'inputs
- Envoie `ClientMessage::Move` au serveur via canal reliable
- Optimisation : envoi seulement si changement > seuil (0.01 rad pour visée)

### `receive_position_updates`

- Reçoit `ServerMessages::NetworkedEntities` (30Hz, unreliable)
- Met à jour `NetworkedTransform.target` pour TOUS les joueurs

### `reconcile_local_player`

- Compare position locale vs position serveur
- Si divergence > 15.0 : correction douce vers serveur
- Évite téléportation brutale, préserve feeling physique

### `interpolate_networked_players`

- Interpole UNIQUEMENT les joueurs distants (pas le local)
- Lissage `Transform` vers `NetworkedTransform.target`
- Vitesse : 25.0 pour compenser latence réseau

## Principe de Fonctionnement

### Joueur Local

1. Physique complète (`RigidBody::Dynamic`)
2. Calcul mouvement basé sur inputs locaux
3. Réconciliation douce avec position serveur si divergence

### Joueurs Distants

1. Physique statique (`RigidBody::Static`) pour être poussables
2. Pas de calcul mouvement local
3. Interpolation pure vers positions serveur

### Obstacles/Murs

1. `RigidBody::Static` pour collision
2. Nécessaire pour que le joueur local Dynamic ne passe pas au travers

## Flux de Données

```
Input Clavier/Souris
    ↓
record_player_directional_input
    ↓
PlayerInput (Resource)
    ↓
input_sync_system → [Réseau] → Serveur
    ↓
Serveur calcule physique
    ↓
[Réseau] ← broadcast_player_positions (30Hz)
    ↓
receive_position_updates
    ↓
NetworkedTransform.target
    ↓
┌─────────────────────┬──────────────────────┐
│ Joueur Local        │ Joueurs Distants     │
│ reconcile_local     │ interpolate_networked│
│ (correction douce)  │ (interpolation)      │
└─────────────────────┴──────────────────────┘
```

## Pièges à Éviter

❌ **NE JAMAIS** interpoler la position du joueur local
→ Saccades car conflit physique locale vs interpolation réseau

❌ **NE JAMAIS** oublier RigidBody::Static sur joueurs distants
→ Le joueur local passera au travers

❌ **NE JAMAIS** créer obstacles sans physique côté client
→ Joueur local Dynamic traversera les murs

❌ **NE JAMAIS** ignorer les positions serveur pour le local
→ Divergence permanente, désynchronisation

✅ **TOUJOURS** filter avec `Without<ControlledPlayer>` pour interpolation
✅ **TOUJOURS** ajouter physique statique aux obstacles
✅ **TOUJOURS** réconcilier doucement (pas de téléportation)

