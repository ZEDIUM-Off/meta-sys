# Meta-system

Meta-system est un framework Rust de composition statique. Il assemble des Addons versionnés au
moyen de Capabilities typées sans imposer de runtime au programme obtenu.

## Composition

**System**:
Une composition résolue d’Addons dont les Capabilities obligatoires sont satisfaites et les
conflits explicitement arbitrés.
_Avoid_: application vivante, Runtime, graphe de Components

**System Definition**:
La sélection déclarative des Addons racines et des choix de résolution à compiler en un System.
_Avoid_: workspace généré, manifeste de déploiement

**Addon**:
L’unité atomique et versionnée de composition. Un Addon contient du code Rust ordinaire et publie
un Addon Contract.
_Avoid_: Component, Provider, plugin dynamique

**Addon Contract**:
La signature de composition d’un Addon, formée exclusivement de ses clauses `provides`, `requires`
et `optional`.
_Avoid_: Capability Contract, manifeste de runtime

**Capability**:
Une interface Rust publique, typée et définie par un Addon. Sa compatibilité évolue avec la version
de l’Addon qui la définit.
_Avoid_: permission, service global, Capability versionnée indépendamment

**Resolution**:
La validation qui sélectionne une composition unique d’Addons, satisfait leurs imports de
Capabilities et explique tout conflit.
_Avoid_: Binding public, injection dynamique, découverte de sources

## Clauses d’un Addon Contract

**provides**:
La clause par laquelle un Addon expose au System une implémentation d’une Capability.
_Avoid_: Provider, réexport implicite

**requires**:
La clause par laquelle un Addon importe obligatoirement une Capability déjà exposée dans le
System.
_Avoid_: Requirement, dépendance optionnelle

**optional**:
La clause par laquelle un Addon peut importer une Capability déjà exposée tout en garantissant
qu’il reste fonctionnel lorsqu’elle est absente.
_Avoid_: dépendance installée automatiquement, fallback implicite
