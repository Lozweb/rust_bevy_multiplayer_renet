# Instructions Copilot - Projet Bevy Multiplayer

Ce fichier contient les règles et bonnes pratiques pour le développement de ce projet.

## Contexte du projet

Jeu multijoueur 2D développé avec :

- **Bevy 0.17** (moteur de jeu ECS)
- **Renet** (networking)
- **Avian2D** (physique 2D)

Architecture : **Full Server Authority**

- Serveur = source de vérité unique
- Clients = affichage de ce que le serveur dit
- Latence acceptable : 50-100ms

## Règles obligatoires

Voir `copilot-instructions.md` pour toutes les règles détaillées.

### Règles critiques

1. **Vérifier imports** après chaque modification
2. **Compiler systématiquement** : `cargo check --all`
3. **Full Server Authority** : pas de physique côté client (sauf joueur local)
4. **Documentation en français**
5. **Zéro warnings** avant de terminer

## Architecture

```
SERVEUR (Autoritaire)
└─ Physique pour TOUS (joueurs + ennemis)
   └─ Broadcast 30Hz (positions unreliable)
   └─ Events reliable (spawn/despawn/mort)

CLIENTS (Display + Input)
└─ Physique joueur local uniquement (avec réconciliation)
   └─ Interpolation pour les autres entités
   └─ Envoie inputs au serveur
```

## Fonctionnalités implémentées

- ✅ Mouvement multijoueur synchronisé (30Hz)
- ✅ Système de tir avec projectiles
- ✅ Ennemis avec santé et types différents
- ✅ Collisions projectiles/ennemis
- ✅ Interpolation fluide (joueurs et ennemis)
- ✅ Réconciliation douce côté client

## Test standard

```bash
# Terminal 1 : Serveur
cargo run --bin server

# Terminal 2 : Client 1
cargo run --bin client --features dev

# Terminal 3 : Client 2
cargo run --bin client --features dev
```

## Fichiers de documentation

- `copilot-instructions.md` - Règles complètes de développement
- `ROADMAP.md` - Fonctionnalités implémentées et à venir
- `BEVY_0_17.md` - Instructions spécifiques Bevy 0.17

## Structure du projet

```
game_core/      Code partagé (components, messages réseau)
server/         Logique autoritaire, physique, IA
client/         Rendu, input, interpolation
```

## Contact

Pour toute question sur l'architecture ou les choix techniques, se référer aux fichiers de documentation listés
ci-dessus.

