# 🗺️ Roadmap - Rust Bevy Multiplayer Renet

## 📌 Décisions d'architecture

### Serveur en mode console uniquement

**Date:** 2025-12-01  
**Décision:** Le serveur fonctionnera **uniquement en mode console** sans interface graphique.

**Raisons:**

- Simplifier le développement
- Réduire les ressources nécessaires
- Faciliter le déploiement sur serveur dédié
- Séparation claire client/serveur

**Impact:**

- Pas de rendu visuel côté serveur
- Pas de dépendances graphiques (Mesh2d, ColorMaterial, etc.)
- Logging console pour le debugging
- Possibilité d'ajouter une UI console/debug textuelle si nécessaire
- Configuration `AssetPlugin` avec `watch_for_changes_override: Some(false)` pour éviter les warnings de file watcher

---

## ✅ Système de tir coopératif - IMPLÉMENTÉ

### État de l'implémentation

**1. Composants partagés (game_core)** ✅

- ✅ `game_core/src/enemy.rs` : Composant `Enemy`, `EnemyType`, `EnemyServerEntity`
- ✅ `game_core/src/projectile.rs` : Composant `Projectile`, `ProjectileLifeTime`
- ✅ Système de santé intégré dans `Enemy` (health, apply_damage, is_dead)
- ✅ Bundles pour spawn facile: `enemy_bundle()`, `projectil_bundle()`

**2. Système d'ennemis (serveur)** ✅

- ✅ `server/src/handler/enemy_event.rs` : Envoi ennemis existants aux nouveaux joueurs
- ✅ `game_core/src/level.rs` : `spawn_initial_enemies()` - spawn 3 ennemis au démarrage
- ✅ Gestion santé et despawn dans collision_event
- ✅ Physique complète : RigidBody::Dynamic, Collider, Mass, LinearDamping

**3. Système de projectiles (serveur)** ✅

- ✅ `server/src/handler/player_input.rs` : `handle_shoot()` - spawn basé sur input
- ✅ `game_core/src/projectile.rs` : `spawn_projectil()` avec physique Avian2D
- ✅ Physique projectile : RigidBody::Dynamic, vélocité basée sur AimDirection
- ✅ Timer de vie (`ProjectileLifeTime`) pour despawn automatique

**4. Gestion des collisions (serveur)** ✅

- ✅ `server/src/handler/collision_event.rs` : Système complet de collisions
- ✅ Écoute `CollisionStart` via `MessageReader`
- ✅ Dégâts projectile → ennemi avec `apply_damage()`
- ✅ Despawn projectile sur impact (évite double-despawn avec HashSet)
- ✅ Broadcast `EnemyDeath` et `ProjectileCollision`

**5. Synchronisation réseau** ✅

- ✅ `game_core/src/server.rs` : Enums de messages réseau
    - `EnemyMessages` : EnemySpawned, EnemyDeath
    - `ProjectileMessages` : ProjectileSpawned, ProjectileCollision, ProjectileCleanup
    - `EnemyPositionMessages` : EnemyPositionsUpdate @ 30Hz
- ✅ `server/src/handler/position_sync.rs` : `sync_enemies_positions()` @ 30Hz
- ✅ Canal reliable (`ServerReliableMessages`) pour spawn/despawn/mort
- ✅ Canal unreliable (`ServerUnreliableMessages`) pour positions

**6. Rendu client** ✅

- ✅ `client/src/client/enemy_event_handler.rs` : Gestion messages ennemis
- ✅ `client/src/client/projectil_event_handler.rs` : Gestion messages projectiles
- ✅ `client/src/client/client_event.rs` : Spawn visuel (Mesh2d, ColorMaterial)
- ✅ `client/src/client/position_sync_event.rs` : `interpolate_networked_enemies()`
- ✅ Rendu sans physique côté client (NetworkedTransform uniquement)

**7. Intégration** ✅

- ✅ Modules exportés dans `game_core/src/lib.rs`
- ✅ Handlers intégrés dans `server/src/handler/mod.rs`
- ✅ Systèmes configurés dans les schedules appropriés
- ✅ Input `shoot` capturé et envoyé au serveur

### Architecture implémentée

**Structure serveur:**

```
server/src/
├── handler/
│   ├── collision_event.rs  ✅ Collisions projectile/ennemi
│   ├── enemy_event.rs      ✅ Synchronisation ennemis
│   ├── player_input.rs     ✅ Input shoot + spawn projectile
│   └── position_sync.rs    ✅ Broadcast positions @ 30Hz
└── level.rs                ✅ Setup niveau + spawn ennemis
```

**Structure game_core:**

```
game_core/src/
├── enemy.rs        ✅ Enemy, EnemyType, spawn_enemy()
├── projectile.rs   ✅ Projectile, spawn_projectil()
├── level.rs        ✅ spawn_initial_enemies()
└── server.rs       ✅ EnemyMessages, ProjectileMessages
```

**Structure client:**

```
client/src/client/
├── enemy_event_handler.rs    ✅ Spawn/despawn ennemis
├── projectil_event_handler.rs ✅ Spawn/despawn projectiles
├── position_sync_event.rs     ✅ Interpolation ennemis
└── system_player_input.rs     ✅ Capture input shoot
```

### Points clés réalisés

- ✅ Filtres de collision: projectiles touchent ennemis (despawn sur impact)
- ✅ Architecture Full Server Authority (physique serveur uniquement)
- ✅ Synchronisation reliable pour spawn/despawn/mort
- ✅ Synchronisation unreliable pour positions @ 30Hz
- ✅ Interpolation fluide des ennemis côté client
- ✅ Système de tir réactif (clic gauche pour tirer)
- ✅ Gestion des dégâts et mort des ennemis
- ✅ Prévention double-despawn avec HashSet
- ✅ Couleurs différenciées par type d'ennemi (Basic=Blanc, Medium=Jaune, Hard=Rouge)

---

## 🎯 Prochaines fonctionnalités à implémenter

### 1. Amélioration du système d'ennemis

**Priorité:** Haute  
**Statut:** Planifié

**Objectifs:**

- [ ] IA de base pour les ennemis (suivre joueur le plus proche)
- [ ] Système de spawn par vagues (remplacer spawn fixe)
- [ ] Augmentation progressive de la difficulté
- [ ] Limites de spawn (éviter spam infini)

**Fichiers à créer/modifier:**

- `game_core/src/enemy.rs` : Ajout IA component
- `server/src/handler/enemy_ai.rs` : Système de suivi joueur
- `server/src/handler/wave_system.rs` : Gestion vagues

---

### 2. Cooldown et feedback de tir

**Priorité:** Moyenne  
**Statut:** Planifié

**Objectifs:**

- [ ] Cooldown entre tirs (éviter spam)
- [ ] Feedback visuel du cooldown côté client
- [ ] Son de tir (optionnel)
- [ ] Particules pour projectiles (optionnel)

**Fichiers à créer/modifier:**

- `game_core/src/player.rs` : Ajout `ShootCooldown` component
- `server/src/handler/player_input.rs` : Vérifier cooldown avant tir
- `client/src/game/player.rs` : UI cooldown

---

### 3. Optimisation réseau

**Priorité:** Moyenne  
**Statut:** Planifié

**Objectifs:**

- [ ] Object pooling pour projectiles (éviter despawn/spawn constant)
- [ ] Compression des messages réseau si nécessaire
- [ ] Culling spatial (ne pas envoyer ennemis hors écran)
- [ ] Mesure de performances réseau (metrics)

---

## 🔮 Fonctionnalités futures

### Mode spectateur

**Priorité:** Moyenne  
**Statut:** Planifié

**Description:**
Permettre à des utilisateurs de se connecter au serveur en tant que spectateurs pour observer une partie en cours sans y
participer.

**Fonctionnalités prévues:**

- Connexion en mode spectateur (sans spawner de joueur)
- Caméra libre ou suivi de joueurs
- Réception des mêmes updates réseau que les joueurs
- UI différenciée pour les spectateurs
- Possibilité de switcher entre différents joueurs
- Chat spectateur (optionnel)

**Considérations techniques:**

- Nouveau type de connexion: `ClientType::Player` vs `ClientType::Spectator`
- Gestion des permissions (spectateurs ne peuvent pas envoyer d'inputs)
- Optimisation bande passante (spectateurs peuvent avoir rate réduit)
- Limite du nombre de spectateurs par serveur

**Dépendances:**

- Système de jeu de base fonctionnel ✅
- Système de caméra flexible (à améliorer)
- UI pour switcher entre modes

---

## 📝 Autres idées futures

### Système de waves d'ennemis

- Remplacer le spawn aléatoire par des vagues structurées
- Difficulté progressive
- Objectifs de survie

### Système d'armes multiples

- Différents types d'armes
- Munitions
- Cooldown de tir
- Armes spéciales (AOE, sniper, etc.)

### Power-ups coopératifs

- Bonus ramassables
- Effets temporaires
- Buffs d'équipe

### Statistiques et progression

- Score par joueur
- Achievements
- Système de progression (optionnel)

### Audio et effets visuels

- Sons de tir, impact, explosion
- Particules pour projectiles
- Feedback visuel pour dégâts
- Animations de mort d'ennemis

### Optimisations

- Object pooling pour projectiles
- Spatial partitioning pour collisions
- Compression des messages réseau
- Prédiction côté client

---

## 📚 Notes de développement

### Architecture client-serveur

- **game_core**: Code partagé (composants, messages réseau)
- **server**: Logique autoritaire, physique, IA
- **client**: Rendu, input, interpolation

### Bibliothèques principales

- **Bevy**: Moteur de jeu ECS
- **Renet**: Networking
- **Avian2D**: Physique 2D
- **Serde**: Sérialisation réseau

### Workflow de développement

1. Définir composants partagés dans `game_core`
2. Implémenter logique autoritaire dans `server`
3. Créer systèmes de synchronisation réseau
4. Implémenter rendu dans `client`
5. Tester et itérer

---

*Dernière mise à jour: 2025-12-01*

