# Rust Bevy Multiplayer avec Renet

Jeu multijoueur développé avec **Bevy 0.17** et **Renet** pour le networking.

## 🎮 Fonctionnalités

- ✅ **Architecture Client-Serveur** avec synchronisation réseau
- ✅ **Mode Headless** pour serveur de production sans interface graphique
- ✅ **Mode Graphique** pour développement local avec debug UI
- ✅ **ECS (Entity Component System)** avec Bevy
- ✅ **Networking UDP** avec Renet et QUIC
- ✅ **Hot-reload** des assets en développement

## 📦 Structure du Projet

```
rust_bevy_multiplayer_renet/
├── game_core/          # Logique de jeu partagée (client + serveur)
│   ├── src/
│   │   ├── client.rs   # Messages client
│   │   ├── server.rs   # Messages serveur
│   │   ├── player.rs   # Composants joueur
│   │   ├── network.rs  # Configuration réseau
│   │   └── transport.rs
│   └── Cargo.toml
├── server/             # Serveur de jeu
│   ├── src/
│   │   ├── main.rs     # Point d'entrée avec mode headless
│   │   ├── config.rs   # Configuration CLI
│   │   ├── plugin/     # Plugins serveur
│   │   ├── system/     # Systèmes ECS
│   │   └── resource/   # Ressources Bevy
│   └── Cargo.toml
├── client/             # Client de jeu
│   ├── src/
│   │   ├── main.rs     # Point d'entrée client
│   │   ├── plugin/     # Plugins client
│   │   └── system/     # Systèmes ECS
│   └── Cargo.toml
└── assets/             # Assets partagés
```

## 🚀 Démarrage Rapide

### Prérequis

- Rust 1.75+ (stable)
- Cargo

### Installation

```bash
git clone https://github.com/votre-repo/rust_bevy_multiplayer_renet.git
cd rust_bevy_multiplayer_renet
```

### Développement Local

```bash
# Terminal 1: Lancer le serveur en mode graphique
cargo run --bin server

# Terminal 2: Lancer le client
cargo run --bin client
```

### Production (Mode Headless)

```bash
# Build optimisé
cargo build --release --bin server

# Lancer le serveur sans interface graphique
./target/release/server --headless --port 5000
```

## 🎛️ Options CLI du Serveur

```bash
# Afficher l'aide
cargo run --bin server -- --help

# Mode headless (production)
cargo run --bin server -- --headless

# Spécifier un port
cargo run --bin server -- --port 7777

# Mode headless avec port personnalisé
cargo run --bin server -- --headless --port 7777
```

## 🔧 Modes du Serveur

### Mode Graphique (Développement)

- Interface de débogage avec fenêtre
- Console de logs visuelle
- Caméra de debug
- Rendu des entités
- Plugins complets de Bevy

**Usage:** `cargo run --bin server`

### Mode Headless (Production)

- Aucune interface graphique
- Plugins minimaux uniquement
- Optimisé pour performances
- Idéal pour serveurs dédiés
- Logs via terminal

**Usage:** `cargo run --bin server -- --headless`

## 🐳 Déploiement Docker

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/
EXPOSE 5000/udp
CMD ["server", "--headless", "--port", "5000"]
```

**Build et Run:**

```bash
docker build -t bevy-multiplayer-server .
docker run -p 5000:5000/udp bevy-multiplayer-server
```

## 🛠️ Développement

### Compilation

```bash
# Compiler tout le workspace
cargo build --workspace

# Compiler uniquement le serveur
cargo build --bin server

# Compiler uniquement le client
cargo build --bin client

# Build optimisé
cargo build --release --workspace
```

### Tests

```bash
# Exécuter tous les tests
cargo test --workspace

# Tests d'un crate spécifique
cargo test -p game_core
```

### Vérification du Code

```bash
# Clippy (linter)
cargo clippy --workspace

# Formatage
cargo fmt --all

# Vérification sans compilation
cargo check --workspace
```

## 🏗️ Architecture Technique

### ECS (Entity Component System)

Le projet utilise l'architecture ECS de Bevy :

- **Entities** : Joueurs, projectiles, etc.
- **Components** : `Transform`, `PlayerInfo`, `Mesh2d`, etc.
- **Systems** : Logique de jeu, networking, rendering

### Networking

- **Protocole** : UDP via Renet (QUIC)
- **Sérialisation** : Bincode
- **Canaux** :
    - `ServerMessages` : Événements serveur → client
    - `ClientMessages` : Inputs client → serveur
    - `NetworkedEntities` : Synchronisation d'état

### Plugins

**Serveur:**

- `ServerPlugin` : Logique serveur principale
- `DebugPlugin` : Interface de debug (mode graphique)

**Client:**

- `ClientPlugin` : Connexion et synchronisation
- `InputPlugin` : Gestion des entrées utilisateur

## 📊 Performances

### Mode Headless vs Graphique

| Métrique  | Headless | Graphique |
|-----------|----------|-----------|
| CPU Usage | ~5%      | ~20%      |
| RAM Usage | ~50 MB   | ~200 MB   |
| Plugins   | Minimal  | Full      |
| Rendu     | ❌        | ✅         |

## 🤝 Contribution

Les contributions sont les bienvenues ! Voir le guide de contribution (à venir).

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 🔗 Ressources

- [Bevy Engine](https://bevyengine.org/)
- [Renet Networking](https://github.com/lucaspoffo/renet)
- [Rust Language](https://www.rust-lang.org/)

## 📝 Changelog

### [1.0.0] - 2025-11-23

#### Ajouté

- Mode headless pour serveur de production
- Configuration CLI avec clap
- Documentation complète
- Support Docker

#### Modifié

- Architecture serveur pour support dual-mode
- Ressources optionnelles pour compatibilité headless

---

**Développé avec ❤️ en Rust + Bevy**

