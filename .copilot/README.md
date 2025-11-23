# Instructions Copilot - Projet Bevy Multiplayer

Ce fichier contient les règles et bonnes pratiques pour le développement de ce projet.

## Contexte du projet

Jeu multijoueur 2D développé avec :

- **Bevy** (moteur de jeu)
- **Renet** (networking)
- **Avian2D** (physique)

Architecture : **Full Server Authority**

- Serveur = source de vérité unique
- Clients = affichage de ce que le serveur dit
- Latence acceptable : 50-100ms

## Règles obligatoires

Voir `.copilot/system_prompt.txt` pour toutes les règles détaillées.

### Règles critiques

1. **Vérifier imports** après chaque modification
2. **Compiler systématiquement** : `cargo check --all`
3. **Full Server Authority** : pas de physique côté client
4. **Documentation en français**
5. **Zéro warnings** avant de terminer

## Architecture

```
SERVEUR (Autoritaire)
└─ Physique pour TOUS
   └─ Broadcast 20Hz

CLIENTS (Display)
└─ Interpolation vers serveur
   └─ Aucune physique propre
```

## Test standard

```bash
cargo run --bin server                    # Terminal 1
cargo run --bin client --features dev     # Terminal 2
cargo run --bin client --features dev     # Terminal 3
```

## Fichiers de documentation

- `SESSION_COMPLETE.md` - Résumé complet de toutes les améliorations
- `RESOLUTION_FINALE.md` - Solution finale de désynchronisation
- `SOLUTION_DEFINITIVE.md` - Architecture détaillée
- Divers `CORRECTION_*.md` - Historique des corrections

## Contact

Pour toute question sur l'architecture ou les choix techniques, se référer aux fichiers de documentation listés
ci-dessus.

