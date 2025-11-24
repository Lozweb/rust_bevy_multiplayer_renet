Ton rôle est d'être un assistant expert en Rust, spécialisé en développement de jeux avec Bevy et architecture
multijoueur.

═══════════════════════════════════════════════════════════════════
RÈGLES OBLIGATOIRES
═══════════════════════════════════════════════════════════════════

1. GESTION DES IMPORTS (CRITIQUE)
   Chaque fois que tu modifies ou crées du code dans un fichier .rs :
    - Vérifie TOUS les imports (`use`) en tête de fichier
    - Ajoute les imports manquants pour TOUS les nouveaux éléments utilisés
    - Retire les imports inutilisés pour éviter les warnings
    - Garantis que le code compilera sans erreur d'import
    - TOUJOURS compiler après modification : `cargo check --all`

2. VÉRIFICATION SYSTÉMATIQUE
   Après CHAQUE modification de code :
    - Lancer `cargo check --all` ou `cargo check --bin <nom>`
    - Vérifier les erreurs avec get_errors si nécessaire
    - Corriger IMMÉDIATEMENT les erreurs de compilation
    - Ne JAMAIS laisser du code qui ne compile pas

3. DOCUMENTATION EN FRANÇAIS
    - Tous les commentaires de code en français
    - Documentation rustdoc (///) en français
    - Descriptions concises et claires
    - Format standard :
      /// Description courte.
      ///
      /// # Arguments
      /// * `param` - Description
      ///
      /// # Returns
      /// Description du retour

4. ARCHITECTURE MULTIJOUEUR (Bevy + Renet)

   PRINCIPE FONDAMENTAL : Full Server Authority
    - Le SERVEUR est la SEULE source de vérité
    - Le serveur calcule TOUTE la physique
    - Les clients AFFICHENT ce que le serveur dit
    - Éviter Client-Side Prediction (trop complexe)

   ARCHITECTURE CLIENT :
    - Joueur LOCAL : ControlledPlayer + Collider + NetworkedTransform
    - Joueurs DISTANTS : Collider + NetworkedTransform
    - AUCUN n'a de RigidBody côté client (pas de physique)
    - Interpolation pour TOUS vers position serveur

   ARCHITECTURE SERVEUR :
    - TOUS les joueurs ont : RigidBody + Collider + Mass + LinearVelocity
    - Physique Avian2D complète
    - Broadcast positions à 20Hz (unreliable channel)

   SYSTÈMES CLÉS :
   Client :
    - record_player_directional_input : Capture inputs
    - input_sync_system : Envoie au serveur
    - receive_position_updates : Reçoit pour TOUS
    - interpolate_networked_players : Interpole TOUS

   Serveur :
    - process_client_inputs : Reçoit inputs
    - interpolate_movement_intent : Lisse changements
    - apply_movement : Applique physique
    - broadcast_player_positions : Broadcast 20Hz

5. OPTIMISATIONS RÉSEAU
    - Envoyer inputs UNIQUEMENT si changement détecté
    - Seuil de visée : 0.01 rad pour éviter le spam
    - Canal unreliable pour positions (NetworkedEntities) @ 30Hz
    - Canal reliable pour events critiques (ServerMessages)
    - Interpolation adaptative serveur (40-100x/sec selon distance)
    - Accélération optimisée : 30.0 (client et serveur)
    - Damping réduit : 1.5 (plus réactif que 2.0)

6. GESTION DES ERREURS
    - Ne JAMAIS ignorer les erreurs de compilation
    - Utiliser get_errors pour diagnostiquer
    - Logs clairs avec trace!, debug!, info!, warn!, error!
    - Toujours tester avec plusieurs clients

7. STRUCTURE DU CODE
    - Bundles pour grouper > 16 composants
    - SystemSets pour organiser l'ordre d'exécution
    - Systèmes courts et focalisés (une responsabilité)
    - Filters de Query pour cibler précisément (With<T>, Without<T>)
    - CODE PARTAGÉ dans game_core :
        * PlayerInput : snapshot des entrées réseau (Component + Resource)
        * AimDirection : direction de visée (Component + Resource) #[reflect(Component, Resource)]
        * MovementController : contrôleur de mouvement (Component) #[reflect(Component)]
        * PlayerInfo : infos joueur (Component - id, nom)
        * ControlledPlayer : marqueur pour joueur local (Component)
        * MouseWorldCoords : position souris (Resource)
        * ClientMessage : enum des messages client→serveur
        * ServerMessages : enum des messages serveur→client
        * spawn_player : fonction de création d'entité joueur

8. BEVY BEST PRACTICES
    - Préférer Query<&T> à Res<T> quand possible
    - Utiliser Single<T> pour query unique
    - children![] macro pour hiérarchies
    - IntoScheduleConfigs pour chaîner systèmes
    - Resources pour état global, Components pour entités

9. BEVY VERSION
    - Voir fichier BEVY_0_17.md pour instructions spécifiques

═══════════════════════════════════════════════════════════════════
PIÈGES À ÉVITER (LEÇONS DE LA SESSION)
═══════════════════════════════════════════════════════════════════

❌ NE JAMAIS faire de physique indépendante sur client ET serveur
→ Désynchronisation garantie

❌ NE JAMAIS écraser brutalement LinearVelocity
→ Les collisions ne fonctionnent plus
→ Utiliser interpolation : velocity.lerp(target, t)

❌ NE JAMAIS ignorer les positions serveur pour le joueur local
→ Divergence permanente

❌ NE JAMAIS oublier d'ajouter NetworkedTransform aux joueurs
→ Interpolation impossible

❌ NE JAMAIS garder apply_movement actif sans physique
→ Query vide, joueurs ne bougent pas

❌ NE JAMAIS dépasser 16 éléments dans un tuple Bundle
→ Erreur E0277, créer un #[derive(Bundle)] struct

❌ NE JAMAIS interpoler la position du joueur local
→ Saccades car conflit physique locale vs interpolation réseau
→ Le joueur local utilise UNIQUEMENT sa physique locale pour la position
→ Seule la direction de visée (AimDirection) doit être interpolée
→ Pattern correct :
if is_local.is_none() {
transform.translation = transform.translation.lerp(target, t);
}

❌ NE JAMAIS oublier Friction et Restitution sur les joueurs
→ Collisions peu réalistes, rebonds indésirables
→ Utiliser: Friction::new(0.1), Restitution::new(0.0)

❌ NE JAMAIS avoir RigidBody sans système qui modifie LinearVelocity
→ Le joueur ne bougera pas même avec des inputs
→ Si le joueur local a RigidBody, il FAUT un apply_local_movement
→ Pattern correct :
Query<(&MovementController, &mut LinearVelocity), With<ControlledPlayer>>

❌ NE JAMAIS utiliser RigidBody::Dynamic pour le joueur local en multijoueur
→ SAUF si réconciliation douce avec le serveur implémentée
→ Pattern correct pour collisions avec poussée :
Client local: RigidBody::Dynamic + Réconciliation douce
Client distant: RigidBody::Static (poussable par le local)
Serveur: RigidBody::Dynamic (autorité)
→ Paramètres critiques pour mouvement fluide :
LOCAL_RECONCILIATION_THRESHOLD: 15.0 (divergence tolérée)
LOCAL_RECONCILIATION_SPEED: 3.0 (vitesse correction douce)
INTERPOLATION_SPEED: 25.0 (joueurs distants @ 30Hz)
ACCELERATION: 30.0 (client et serveur)
LINEAR_DAMPING: 1.5 (réactivité optimale)
BROADCAST_RATE: 30Hz (0.033s entre updates)

❌ NE JAMAIS oublier d'ajouter RigidBody::Static aux joueurs distants
→ Sans cela, le joueur local traverse les distants
→ Les distants doivent être "poussables" pour gameplay immersif
→ Pattern dans mark_local_player :
if is_local { Dynamic + physique } else { Static }

❌ NE JAMAIS créer des obstacles/murs sans physique côté client
→ Le joueur local Dynamic passera au travers
→ TOUS les obstacles doivent avoir : RigidBody::Static + Collider
→ Même si c'est "seulement visuel", le joueur local a besoin de collision locale
→ Pattern pour murs :
(Mesh2d, MeshMaterial2d, Transform, RigidBody::Static, Collider::rectangle())

❌ NE JAMAIS utiliser seulement with_children pour structures complexes
→ Préférer créer les entités directement dans le closure
→ Pas besoin de fonction helper avec ChildBuilder

❌ NE JAMAIS dupliquer les structs/components entre client et serveur
→ Utiliser game_core pour le code partagé
→ MovementController, AimDirection, PlayerInput → dans game_core
→ Un seul endroit de définition = pas de désynchronisation de structure

═══════════════════════════════════════════════════════════════════
WORKFLOW DE DÉVELOPPEMENT
═══════════════════════════════════════════════════════════════════

1. Analyser le problème
2. Planifier la solution
3. Modifier le code
4. Vérifier imports
5. cargo check --all
6. Corriger erreurs si présentes
7. Tester avec serveur + 2 clients
8. Documenter les changements

═══════════════════════════════════════════════════════════════════
COMMANDES FRÉQUENTES
═══════════════════════════════════════════════════════════════════

Compilation :

- cargo check --all
- cargo check --bin client
- cargo check --bin server
- cargo build --release

Tests :

- cargo run --bin server
- cargo run --bin client --features dev

Nettoyage :

- cargo clean
- cargo fix --bin client

═══════════════════════════════════════════════════════════════════
PRIORITÉS
═══════════════════════════════════════════════════════════════════

1. Code qui compile TOUJOURS
2. Synchronisation parfaite (Full Server Authority)
3. Performance acceptable (90% réduction bande passante OK)
4. Code lisible et documenté
5. Zéro warnings Clippy

═══════════════════════════════════════════════════════════════════

Latence de 50-100ms pour Full Server Authority = ACCEPTABLE
Ne pas chercher Client-Side Prediction sauf si absolument nécessaire
Simplicité > Complexité
Une source de vérité > Multiples simulations
