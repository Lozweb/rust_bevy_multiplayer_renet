# 🗺️ Roadmap - Rust Bevy Multiplayer Renet

## 📌 Décisions d'architecture

### Serveur en mode console uniquement

**Date:** 2025-01-24  
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

---

## 🎯 Prochaine fonctionnalité: Système de tir coopératif

### Plan d'implémentation

**1. Composants partagés (game_core)**

- [ ] Créer `game_core/src/combat.rs`
- [ ] Définir composants: `Enemy`, `Projectile`, `Health`, `ProjectileOwner`
- [ ] Système de collision layers (bitflags)

**2. Système d'ennemis (serveur)**

- [ ] Créer `server/src/system/enemy.rs`
- [ ] Spawn d'ennemis (aléatoire simple pour MVP)
- [ ] Gestion santé et despawn

**3. Système de projectiles (serveur)**

- [ ] Créer `server/src/system/projectile.rs`
- [ ] Spawn basé sur input `shoot` + `AimDirection`
- [ ] Physique avec Avian2D
- [ ] Despawn automatique (distance/temps max)

**4. Gestion des collisions (serveur)**

- [ ] Créer `server/src/system/collision.rs`
- [ ] Écouter événements `CollisionStarted`
- [ ] Dégâts projectile → ennemi
- [ ] Despawn projectile sur impact
- [ ] Filtres: ignorer joueur ↔ projectile

**5. Synchronisation réseau**

- [ ] Créer `server/src/system/combat_sync.rs`
- [ ] Messages: `ProjectileSpawn/Despawn`, `EnemySpawn/Update/Despawn`
- [ ] Canal fiable pour spawn/despawn
- [ ] Canal non-fiable pour positions (30Hz)

**6. Rendu client**

- [ ] Créer `client/src/game/combat.rs`
- [ ] Réception messages réseau
- [ ] Spawn visuel (Mesh2d, sans physique)
- [ ] Interpolation positions

**7. Intégration**

- [ ] Ajout plugins dans `server/src/server.rs`
- [ ] Ajout plugins dans client
- [ ] Export module combat dans `game_core/src/lib.rs`
- [ ] Configuration ordre des systèmes

### Points clés

- ✅ Filtres de collision: projectiles touchent ennemis + murs (pas joueurs)
- ✅ Architecture autoritaire côté serveur
- ✅ Synchronisation fiable pour spawn/despawn
- ✅ Hooks pour feedback visuel/audio (à implémenter après)

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

*Dernière mise à jour: 2025-01-24*

