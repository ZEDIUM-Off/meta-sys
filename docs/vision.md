# Vision du projet

## Thèse

Un logiciel devrait pouvoir capitaliser sur des interfaces et des compositions réutilisables sans
transformer chaque nouveau produit en nouveau framework. Meta-system fournit un langage Rust
commun pour assembler des Addons versionnés au moyen de Capabilities typées.

```text
Addon A
    provides X

Addon B
    requires X
    provides Y

Addon C
    requires Y
```

Un System est la fermeture résolue de cet assemblage. Meta-system vérifie sa cohérence pendant la
compilation, puis le code s’exécute comme du Rust ordinaire.

## Capitalisation

Une Capability stabilisée devient une primitive disponible pour les futurs Addons. Un Addon étend
le langage du System en utilisant des Capabilities existantes pour en exposer de nouvelles, sans
hériter des Addons précédents ni modifier leur code.

```text
ByteWrite → Log → StructuredLog → Audit
```

Cette récursion permet de construire progressivement des familles cohérentes d’Addons pour le CLI,
les processus, les services, le réseau, les interfaces ou de nouveaux paradigmes comme un mesh.
Les Addons de liaison entre ces domaines restent des Addons ordinaires décrits par les mêmes trois
clauses.

## Frontière

Meta-system compose du code; il ne remplace pas le code.

Le framework n’impose ni machine à états, ni Event, ni scheduler, ni conteneur global, ni chargement
dynamique. Un Addon reste libre d’employer Tokio, des threads, des acteurs, des callbacks ou aucune
infrastructure particulière. Un runtime, un gestionnaire de processus ou un daemon peuvent être
apportés par des Addons sans devenir des primitives de Meta-system.

Cargo reste responsable des crates et de leur compilation. Meta-system ajoute la résolution
sémantique des Capabilities et des Addon Contracts. Il ne réécrit pas le workspace utilisateur et
ne génère pas le code métier du System.

## Unités de version

Seuls les Addons sont versionnés. Toutes les Capabilities publiques définies par un Addon font
partie de son interface versionnée; une rupture de l’une d’elles impose une version majeure de cet
Addon.

Les sources, registries, révisions Git et caches ne relèvent pas du composeur. Un futur gestionnaire
d’Addons pourra les préparer avant la compilation, idéalement en s’appuyant sur Cargo et un store
partagé plutôt qu’en reconstruisant un package manager complet.

## Exemples de Systems

Un outil terminal peut composer des Addons de flux, CLI, templating et logging. Le CLI et le
logging importent la même Capability d’écriture, exposée une seule fois, tandis que le logging
construit une nouvelle Capability `Log` au-dessus d’elle.

Un System de services peut ajouter processus, daemon et SSH. Des Addons supplémentaires peuvent
alors exposer des Capabilities de nœud, mesh ou placement, sans que Meta-system connaisse ces
domaines. Les surfaces CLI ou web restent optionnelles en ajoutant seulement les Addons qui les
relient aux Capabilities existantes.

## Non-objectifs

- Définir le runtime universel d’un programme.
- Remplacer Cargo, rustc ou le code Rust ordinaire.
- Découvrir magiquement des crates absentes du graphe de dépendances préparé pour Cargo.
- Versionner chaque Capability indépendamment de son Addon propriétaire.
- Introduire `Component`, `Provider`, `Requirement` ou `Binding` dans le langage public.
- Figer maintenant un registry, un format de package, une ABI dynamique ou un modèle distribué.

## Direction proche

Le prochain travail consiste à valider la plus petite interface Rust directement compilable pour:

1. définir une Capability;
2. déclarer un Addon Contract;
3. exposer une implémentation avec `provides`;
4. recevoir des imports `requires` et `optional` typés;
5. résoudre une chaîne et un diamant sans duplication;
6. produire des diagnostics explicites pour les absences, collisions, cycles et versions
   incompatibles.

Le prototype runtime-first précédent reste une exploration historique. Il ne contraint plus cette
interface.
