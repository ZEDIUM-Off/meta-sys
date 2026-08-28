# Direction du Kernel

Ce document fixe la direction architecturale actuelle. Il borne un premier Kernel volontairement petit sans figer son API Rust.

## Objet

Le Kernel est le langage et l’orchestrateur, neutres vis-à-vis des domaines, d’un graphe de machines à états dynamiques. Il héberge, résout et fait évoluer l’unique System Graph d’un Runtime, mais ne possède aucune Capability métier ou tournée vers le monde extérieur.

Chaque Component est une machine à états. Le problème central est l’orchestration et la résolution incrémentales et efficaces de tous leurs états et relations, au travers d’interfaces communes intuitives pour les auteurs de Components et d’Addons.

Des applications différentes peuvent partager le même Kernel avec des Addons différents. Une application autonome possède un Runtime; réunir plusieurs compositions dans une application signifie employer un seul Runtime. La communication éventuelle entre Runtimes relève de futurs Addons, pas du Kernel.

## Modèle d’extension

**Addon** est le terme canonique. Une même unité peut cumuler plusieurs spécialisations:

- un **Loader Addon** étend le cycle de vie du Loader;
- un **System Addon** contribue au System sans élargir les connaissances du Kernel;
- un **Runner Addon** étend le cycle d’exécution ou de supervision.

`Extension` n’est pas un concept concurrent. Un Addon peut étendre les machines à états introduites par d’autres Addons avec les mêmes mécanismes ordinaires.

## Invariants

### Une base sans autorité extérieure

Sans Addon, le Kernel n’expose ni filesystem, ni réseau, ni shell, ni processus, ni horloge, ni stockage, ni interface utilisateur, ni journalisation. Le Loader ou l’application hôte peuvent employer ces moyens pour le bootstrap sans les accorder implicitement aux Components.

```text
ENGINE != CAPABILITY
```

Une Capability et un Requirement sont des objets inspectables de composition. Un Requirement exprime le besoin d’une Capability et jamais une autorisation.

Les permissions et politiques sont fournies par des Addons. Le chargement et le Binding offrent chacun un seam global minimal permettant à ces Addons d’observer, d’admettre, de rejeter ou d’influencer l’opération; sans Addon de politique, le comportement est `allow-all`.

### Des primitives fermées, des sémantiques ouvertes

Le Kernel conserve peu de primitives. Les Addons ajoutent des sémantiques et peuvent étendre d’autres Addons sans agrandir le vocabulaire propre au Kernel.

Un concept n’est une primitive que s’il est neutre vis-à-vis des domaines, interprété génériquement par le Kernel et représenté dans le System Graph. Les structures de données du scheduler, executors, registres et mécanismes de synchronisation restent des détails d’implémentation.

### Un seul graphe vivant

Definitions, Instances, Requirements, Capabilities, Bindings, Facets, Effects et états de cycle de vie coexistent dans un unique System Graph dynamique. Il n’existe pas de graphes « desired » et « observed » séparés.

Les deux états de résolution stables d’une Component Instance suivent la distinction de Cordis:

```text
Pending — au moins un Requirement nécessaire n’est pas résolu
Active  — les Bindings nécessaires existent et son Component Runtime vit
```

`Activating` et `Deactivating` sont des transitions observables; `Failed` décrit un échec de cycle de vie, pas un troisième état de résolution. La résolution est portée par les Bindings explicites, jamais par un état `Resolved` séparé.

### Definitions, Instances et relations explicites

Le modèle ne confond pas:

```text
Component Definition != Component Instance
Component Instance   != Component Runtime
Facet Schema         != Facet
Capability Contract  != Capability
Requirement          != Binding
```

Le Loader charge une Component Definition complète. Son identité déclarative et toutes ses contributions sont inspectables par l’interface centrale avant que ses Instances ne vivent.

Chaque Binding relie explicitement un Requirement à un fournisseur. Il peut apparaître, disparaître ou changer lorsque le graphe évolue, et demeure inspectable pour expliquer la sélection et la réactivation.

Chaque Facet Schema appartient à l’Addon qui en définit le sens. Chaque Effect appartient à la Component Instance qui l’introduit afin que sa désactivation permette de le retirer, le compenser ou le classer explicitement.

### Résolution incrémentale et exécution parallèle sûre

Le Resolver recalcule uniquement les entités affectées par une mutation du graphe. Les dépendances de lecture et d’écriture sont explicites; elles déterminent quels travaux sont indépendants et lesquels doivent être ordonnés.

Le scheduler garantit un ordre local déterministe lorsque des dépendances ou hooks le déclarent et permet l’exécution concurrente du reste. Le modèle n’admet donc ni mutex global, ni file séquentielle globale, ni hypothèse de thread unique. Un premier executor séquentiel peut servir de stratégie de référence, mais l’interface et les invariants restent parallèles dès l’origine, avec une rigueur comparable à celle d’un scheduler de moteur de jeu sans vocabulaire propre au jeu.

### Déterminisme local

Le Kernel garantit l’ordre FIFO d’acceptation et de distribution au sein d’une Room, ainsi que l’ordre déterministe des hooks là où il est déclaré. Il n’invente aucun ordre causal entre Rooms.

Un Component qui exige un ordre sémantique le valide et le traite lui-même. Recevoir `ProviderRemoved` avant `ProviderRegistered` est une condition de niveau Component, pas une anomalie que le Kernel répare globalement.

### Évolution par Events

Loader, Kernel Runtime, Component Runtime et Runner sont des machines à états événementielles extensibles par des Addons:

```text
Current State + Event → Next State
```

Une opération d’API peut produire un Event et un Addon peut envoyer directement des Events. Il n’existe ni `Command`, ni buffer de commit spécial: une réaction poursuit l’évolution en envoyant d’autres Events par l’API ordinaire.

## Cycles de vie et chargement

Le Loader et les Runtimes sont des machines à états extensibles pilotées uniquement par des Events. Le cycle du Loader possède des phases ordonnées:

```text
Declared
→ Located
→ Materialized
→ Inspected
→ Admitted | Rejected
→ Registered
→ Ready
```

`Materialized` charge le support exécutable; `Inspected` obtient ensuite la Component Definition complète. Le Kernel ne définit pas de descripteur partiel concurrent. Les hooks actifs observent ou influencent les transitions applicables selon un ordre total déclaré et déterministe, sans réordonner ni contourner les phases. Les Addons peuvent ajouter des états et des Events aux points d’extension déclarés. Sans hook de politique, le chargement est admis.

L’ordre de configuration expose la limite du bootstrap: un Loader Addon ne peut gouverner aucune phase déjà passée avant qu’il devienne `Active`. Une politique nécessaire dès l’origine doit donc faire partie du bootstrap de confiance déclaré par l’intégrateur.

Un même Addon peut couvrir Loader et System et corréler lui-même ses contributions. Le Kernel n’exige ni corrélation, ni identité partagée, ni protocole entre elles.

### Chargement natif et sécurité

Suivant le compromis pragmatique de `cordis-rs`, le bootstrap du Loader peut utiliser le filesystem pour ouvrir une bibliothèque dynamique native. Ce mécanisme privé n’expose aucune Capability filesystem. Le chargement peut déjà exécuter du code natif avant que la Component Definition complète soit inspectable; l’ordre des phases rend ce fait explicite plutôt que de promettre une admission préalable impossible.

Du Rust natif exécuté dans le processus peut contourner les Capabilities déclarées par `std`, FFI ou appels système. Les Requirements gardent leur valeur pour la composition, la substitution et la réutilisation, mais ne forment pas une sandbox. Reconstruire depuis les sources est au plus une politique de chaîne d’approvisionnement. Permissions, politiques, isolation et sécurité relèvent d’Addons ou de l’intégrateur.

## Modèle commun d’exécution

Le modèle commun sépare:

```text
Component Definition — identité et contributions déclaratives complètes
Component Instance   — occurrence vivante et identité de son Current State
Component Runtime    — état d’exécution et Mailbox attachés à l’Instance
EventLoopDriver      — interface commune pour faire avancer et arrêter l’exécution
```

Chaque Component fournit par cette interface une stratégie d’arrêt. Sa désactivation arrête son Component Runtime, libère son état d’exécution et sa Mailbox, retire ses Effects et refuse les nouveaux envois vers les Rooms qu’il possède. Une réactivation crée de nouvelles instances conceptuelles de ces Rooms; leur adressage logique stable conserve toutefois la validité des références d’envoi existantes.

Le Runner choisit une stratégie d’exécution conforme à ces contrats. Un executor séquentiel peut être la première stratégie de référence, jamais une contrainte architecturale.

## Routage des Events

Il n’existe qu’un concept de message, **Event**, avec trois opérations:

- `send` adresse une Room;
- `emit` publie depuis un Component vers ses abonnés selon le contrat déclaré;
- `broadcast` atteint les listeners de broadcast.

Chaque Room possède une file de distribution FIFO bornée. Elle accepte les Events dans leur ordre de réception et exécute une seule étape de distribution à la fois. Des Rooms différentes et les autres travaux indépendants peuvent progresser simultanément.

Une Subscription distribue vers la Mailbox bornée du Component destinataire. Le Component possède, au travers de l’interface commune, sa stratégie de retrait, de débordement et d’exécution. Le Kernel ne réessaie jamais automatiquement une Delivery; un destinataire peut remettre en file ou renvoyer un Event par les opérations ordinaires.

Pour le prototype, chaque `RoutingContract` déclare un `MailboxPolicy` réunissant une capacité strictement positive et une stratégie de débordement. Le défaut est `RejectNew`: la nouvelle Delivery est refusée et les Deliveries en attente restent intactes. `DropOldest` est l’autre stratégie de référence: la plus ancienne Delivery en attente est retirée et la nouvelle est acceptée une seule fois. Ces deux décisions sont visibles dans le Receipt et ne déclenchent ni retry, ni transaction, ni backpressure globale. Toute remise en file ou tout renvoi ultérieur passe par `send`, `emit` ou `broadcast` comme un Event ordinaire.

La disponibilité de la Room est vérifiée au moment de `send`. Son adresse logique peut survivre à sa désactivation, mais l’envoi échoue tant qu’aucune instance active de la Room ne la porte.

Un Send Receipt est une observation: acceptation par la Room, destinataires ayant reçu l’Event et, lorsque le Driver permet de l’observer, l’ayant traité. Il ne promet ni transaction, ni causalité globale, ni nouvelle tentative.

## Ordonnancement

Toute mutation et tout travail déclarent leurs dépendances pertinentes. Le Resolver ne reconsidère que le sous-graphe affecté; le scheduler ordonne les travaux dépendants et rend les travaux indépendants exécutables concurremment.

Les garanties sont locales:

- FIFO d’acceptation et de distribution dans une Room;
- ordre déterministe déclaré des hooks;
- aucun ordre causal entre Rooms.

Le Kernel ne contient ni mutex global, ni file série globale, ni hypothèse de thread unique. Un Component qui exige un ordre sémantique entre Events le valide lui-même. L’objectif est la rigueur d’un scheduler de moteur de jeu, sans introduire de concepts de jeu.

## Seams globaux

Deux seams minimaux permettent aux Addons actifs d’observer, admettre, rejeter ou influencer:

1. les transitions de chargement;
2. la création ou le remplacement d’un Binding.

Leur ordre de hooks est déclaré et déterministe. En l’absence d’Addon participant, les deux seams sont `allow-all`. Ils structurent les politiques sans faire d’une permission un Requirement et sans ajouter de politique métier au Kernel.

## Portée de la première implémentation

La première implémentation comprend seulement:

- le System Graph dynamique avec Definitions, Instances, Capabilities, Requirements, Bindings, Facets, Effects et états `Pending`/`Active`;
- le Resolver incrémental et un planificateur à dépendances explicites;
- un executor séquentiel de référence interchangeable;
- les cycles événementiels extensibles du Loader et du Runtime, avec hooks ordonnés;
- le chargement natif d’une Component Definition complète;
- Rooms, Subscriptions, Mailboxes bornées, Deliveries et Send Receipts;
- l’arrêt commun et le nettoyage de chaque Component.

Elle ne comprend ni Capability tournée vers le monde, ni sandbox, ni communication inter-Runtime, ni catalogue de politiques ou d’Addons.

## Tests d’admission et invariants vérifiables

La première implémentation est admise si les scénarios suivants passent:

1. **Structure parallèle.** Deux travaux sans dépendance apparaissent dans le même front exécutable et peuvent se chevaucher avec un executor concurrent de test; une dépendance explicite impose l’ordre. Aucun état global ne requiert leur sérialisation.
2. **Résolution dynamique.** Sans fournisseur, le consommateur est `Pending`; l’ajout d’une Capability crée un Binding et l’active; son retrait supprime le Binding, nettoie ses Effects et le remet `Pending`; un remplacement peut le réactiver.
3. **Routage local.** Une Room distribue en FIFO, une étape à la fois, vers les Mailboxes abonnées; deux Rooms peuvent progresser en parallèle et aucun test ne dépend d’un ordre entre elles. Le débordement suit la stratégie du Component et ne déclenche aucun retry implicite.
4. **Cycle extensible.** Une opération d’API ou un Addon envoie un Event, les hooks actifs s’exécutent dans leur ordre déclaré et la transition respecte `Current State + Event → Next State`, sans `Command` ni outbox.
5. **Nettoyage et réactivation.** La désactivation appelle la stratégie d’arrêt, libère Runtime et Mailbox et rend les Rooms indisponibles; après réactivation, une ancienne référence logique envoie vers la nouvelle Room conceptuelle.
6. **Bootstrap et politiques.** Sans Addon, chargements et Bindings sont admis. Un Addon actif peut les rejeter ou les influencer, mais ne gouverne aucune phase antérieure à son activation.
7. **Receipt.** Le compte rendu distingue acceptation, réception et traitement observable sans provoquer de transaction ni de retry.

## Questions ouvertes

Ces choix restent volontairement différés jusqu’à l’implémentation:

- structures de données exactes du scheduler et du Resolver;
- réglage futur des capacités selon les profils d’intégration;
- ABI dynamique entre le Loader et une bibliothèque native.
