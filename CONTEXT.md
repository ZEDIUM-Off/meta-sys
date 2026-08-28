# Meta-system

Meta-system est un petit langage et orchestrateur, neutre vis-à-vis des domaines, pour composer des machines à états dynamiques.

## Modèle de composition

**Meta-system**:
L’environnement de composition dans lequel des Systems existent et évoluent. Ce terme désigne actuellement le projet, sans préjuger de son nom public final.
_Avoid_: navigateur distribué, framework d’agents, système d’exploitation

**System**:
Une composition vivante de Components dont l’identité ne dépend pas de la topologie qui l’exécute.
_Avoid_: dépôt, déploiement

**System Graph**:
L’unique graphe dynamique qui représente un System, ses entités vivantes et leurs relations de résolution.
_Avoid_: graphe désiré, graphe observé, conteneur de dépendances

**Runtime**:
L’exécution vivante d’une machine à états, faisant évoluer un Current State en réponse à des Events.
_Avoid_: processus, thread, objet simplement mutable

**Kernel**:
Le langage et l’orchestrateur neutres qui hébergent, résolvent et font évoluer un System Graph sans capacité propre tournée vers le monde extérieur.
_Avoid_: plateforme, bibliothèque standard, runtime distribué

**Kernel Runtime**:
L’évaluation vivante d’exactement un System Graph. Des applications autonomes utilisent des Kernel Runtimes distincts, sauf si elles sont composées en une seule application.
_Avoid_: machine, processus, hôte

## Modèle d’extension

**Addon**:
Une unité qui étend une ou plusieurs machines à états du Meta-system, y compris celles introduites par d’autres Addons.
_Avoid_: Extension, Plugin, module lorsqu’il désigne le concept canonique

**Loader Addon**:
La spécialisation d’un Addon qui étend le cycle de vie du Loader.
_Avoid_: loader plugin, loading Extension

**System Addon**:
La spécialisation d’un Addon qui contribue au System sans élargir les connaissances du Kernel.
_Avoid_: System plugin, Extension

**Runner Addon**:
La spécialisation d’un Addon qui étend le cycle de vie d’exécution et de supervision.
_Avoid_: runner plugin, supervisor Extension

## Modèle de composant

**Component**:
Une machine à états composable dont le comportement est décrit par une Component Definition et vécu par une Component Instance.
_Avoid_: plugin, service

**Component Definition**:
L’identité statique et déclarative complète d’un Component ainsi que ses contributions inspectables, indépendamment de toute occurrence vivante.
_Avoid_: descripteur partiel, plugin class

**Component Instance**:
L’occurrence vivante d’une Component Definition et l’identité de son état courant.
_Avoid_: processus, service instance, plugin instance

**Component Runtime**:
L’exécution attachée à une Component Instance, qui reçoit des Events et maintient l’état appartenant au Component.
_Avoid_: Component Instance, processus, thread

**Runner**:
La machine à états qui dirige l’exécution des Component Runtimes selon une stratégie sélectionnée.
_Avoid_: processus superviseur, thread

**EventLoopDriver**:
L’adaptateur commun qui fait avancer l’exécution d’un élément tout en permettant des stratégies spécialisées.
_Avoid_: boucle d’événements propre à un domaine

**Capability Contract**:
Le contrat sémantique d’une aptitude que des fournisseurs peuvent offrir et des consommateurs demander.
_Avoid_: plugin API, permission

**Capability**:
Une offre inspectable d’un Capability Contract publiée par une Component Instance.
_Avoid_: permission, autorisation, fournisseur

**Requirement**:
Un besoin inspectable de Capability exprimé par un consommateur; il ne constitue jamais une autorisation.
_Avoid_: permission, référence directe au fournisseur

**Binding**:
La relation explicite qui résout un Requirement vers le fournisseur d’une Capability.
_Avoid_: injection, lookup implicite

**Facet Schema**:
La définition, possédée par un System Addon, d’une dimension sémantique pouvant enrichir une entité du Kernel.
_Avoid_: sous-classe, propriété héritée

**Facet**:
L’instance d’un Facet Schema attachée à une entité du System Graph.
_Avoid_: métadonnée arbitraire, champ d’extension non typé

**Context**:
Une portée structurelle qui organise visibilité, possession et cycle de vie dans un System Graph.
_Avoid_: conteneur d’injection, workspace, node

**Effect**:
Une conséquence vivante possédée par la Component Instance qui l’a introduite et régie par son cycle de vie.
_Avoid_: effet de bord non géré

## Modèle événementiel

**Current State**:
L’état présent d’une machine à états, à partir duquel un Event détermine son Next State.
_Avoid_: état désiré

**Next State**:
L’état succédant au Current State après interprétation d’un Event.
_Avoid_: état observé

**Event**:
Le message unique qui provoque ou propage une évolution d’état, sans distinction intrinsèque entre intention et résultat.
_Avoid_: Command, type de message « request » générique

**Room**:
Une portée ordonnée de routage qui accepte des Events et les distribue aux Components abonnés.
_Avoid_: Context, handler

**Mailbox**:
La file bornée propre à un Component qui agrège les livraisons de toutes ses Rooms configurées.
_Avoid_: file d’une Room, boîte réseau

**Subscription**:
La relation par laquelle les livraisons d’une Room sont routées vers la Mailbox d’un Component.
_Avoid_: socket, handler registration

**Delivery**:
La tentative observable de placer dans une Mailbox un Event distribué par une Room.
_Avoid_: copie d’Event, message global

**Send Receipt**:
Le compte rendu inspectable d’un envoi et de sa distribution, indiquant son acceptation ainsi que les Components qui ont reçu l’Event et, lorsque cela est observable, l’ont traité; il n’implique ni nouvelle tentative ni traitement transactionnel.
_Avoid_: accusé transactionnel, promesse de nouvelle tentative

## Modèle de chargement

**Loader**:
La machine à états qui charge des Addons et transmet leurs contributions complètes au Kernel Runtime.
_Avoid_: primitive du Kernel, fournisseur implicite de Capability filesystem
