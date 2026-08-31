# Direction de la composition

Ce document borne la direction architecturale actuelle sans figer encore son interface Rust.

## Objet

Meta-system compose statiquement des Addons versionnés. Chaque Addon publie un Addon Contract qui
déclare les Capabilities qu’il expose, importe obligatoirement ou utilise optionnellement. Une
System Definition sélectionne des Addons racines; la Resolution vérifie que leur fermeture forme un
System unique et compilable.

```text
System Definition
       │
       ▼
Addon Contracts ── Resolution ──▶ System
       │                              │
       └── Rust crates ───────────────┴──▶ Cargo / rustc
```

Le composeur ne reste pas résident dans le programme final. L’exécution appartient au code des
Addons sélectionnés.

## Modèle minimal

### Capability

Une Capability est une interface Rust publique et typée. Elle appartient à l’interface versionnée
de l’Addon qui la définit; elle ne possède pas de version autonome.

Un autre Addon peut exposer une implémentation de cette Capability à condition de compiler contre
la même version de l’Addon propriétaire. Le mécanisme Rust exact — trait, type associé, valeur ou
combinaison de ces formes — reste une question d’interface à résoudre.

### Addon Contract

Le Contract est la table d’imports et d’exports de l’Addon:

```text
addon logging
    provides Log
    requires io::ByteWrite
    optional clock::Timestamp
```

- `provides` expose une implémentation au System;
- `requires` importe exactement une implémentation compatible et rend son absence invalide;
- `optional` importe zéro ou une implémentation déjà présente et ne provoque jamais son ajout.

Ces clauses décrivent la composition. Elles ne prescrivent ni cycle de vie, ni protocole d’appel,
ni modèle d’exécution.

### System

La surface du System est l’union dédupliquée des Capabilities explicitement exposées par les Addons
sélectionnés. Importer une Capability ne la réexporte pas et ne crée aucune copie.

```text
stdio  provides ByteWrite
cli    requires ByteWrite, provides Cli
logger requires ByteWrite, provides Log

System capabilities = { ByteWrite, Cli, Log }
```

Le CLI et le logger utilisent la même implémentation de `ByteWrite`. La manière de construire et
partager une Capability avec état doit rester du Rust statiquement typé, sans imposer de conteneur
dynamique universel.

## Invariants de Resolution

### Contracts disjoints

Pour un même Addon, les trois ensembles sont disjoints:

```text
provides ∩ requires = ∅
provides ∩ optional = ∅
requires ∩ optional = ∅
```

Un Addon ne peut donc pas importer et exposer la même Capability. Une extension doit publier une
nouvelle Capability:

```text
structured-logging
    requires Log
    provides StructuredLog
```

Une future adaptation de version exigera une sémantique explicite plutôt qu’une exception implicite
à cette règle.

### Satisfaction

- chaque `requires` est satisfait par exactement un Addon sélectionné;
- chaque `optional` reçoit zéro ou une implémentation déjà sélectionnée;
- deux implémentations de la même Capability constituent un conflit tant que la System Definition
  n’en sélectionne pas explicitement une;
- un Addon est sélectionné au plus une fois;
- une seule version d’un même Addon est admise dans un System initial;
- les cycles de Capabilities sont refusés dans la première version.

La Resolution peut conserver des liens internes pour expliquer ses décisions. Ces liens ne
deviennent pas un concept public concurrent des clauses du Contract.

## Versionnement

L’Addon est l’unique unité de versionnement Meta-system. Une version mineure peut ajouter une
Capability de manière compatible; une rupture de n’importe quelle Capability publique impose une
version majeure de l’Addon qui la définit.

```text
io@1.4.0
    defines ByteRead
    defines ByteWrite

stdio@3.1.0
    provides io@1::ByteRead
    provides io@1::ByteWrite
```

La notation qualifiée est utile aux diagnostics; le code Rust importe normalement les types de
l’Addon sélectionné. Un digest de Contract peut détecter une version publiée qui aurait été
réécrite, sans devenir un second numéro de version.

## Compilation et sources

Meta-system n’ajoute aucune crate pendant une invocation déjà commencée de `rustc`. Deux parcours
sont envisagés:

1. `cargo build`, lorsque tous les Addons candidats sont déjà déclarés dans le graphe Cargo;
2. `meta build`, lorsqu’un gestionnaire prépare d’abord les sources et métadonnées Cargo dans un
   espace de build isolé, puis invoque Cargo.

Le second parcours ne doit ni modifier le workspace utilisateur ni générer son code métier. Un
manifest Cargo éphémère reste toutefois nécessaire pour compiler des crates qui n’étaient pas dans
le graphe initial.

La gestion des registries, URLs Git, révisions, caches, checksums et versions de packages appartient
à un futur gestionnaire d’Addons, pas au composeur. Cet outil devrait déléguer à Cargo ce que Cargo
sait déjà faire et ne conserver que la couche sémantique propre aux Addons, leur store et leur
lockfile.

## Exemple de capitalisation

```text
stdio
    provides io::ByteRead
    provides io::ByteWrite

cli
    requires io::ByteRead
    requires io::ByteWrite
    optional logging::Log
    provides Cli

logging
    requires io::ByteWrite
    provides Log

templating
    requires Cli
    optional Log
    provides Template
```

Le System expose une fois `ByteRead`, `ByteWrite`, `Cli`, `Log` et `Template`. Le runtime éventuel
de chacune de ces interfaces appartient aux Addons qui les implémentent.

Un Addon peut créer un paradigme supérieur sans mécanisme spécial:

```text
process → service → ssh → mesh.node → mesh.placement
```

Chaque flèche signifie seulement que l’Addon suivant importe des Capabilities existantes et en
expose de nouvelles.

## Tests d’admission de la première interface

1. **Chaîne.** `A provides X`, `B requires X and provides Y`, `C requires Y` forme un System valide.
2. **Diamant.** CLI et Logging importent la même Capability de Stdio sans dupliquer Stdio ni la
   Capability.
3. **Absence.** Un `requires` absent produit un diagnostic lié à l’Addon et à la Capability.
4. **Optionalité.** Le même Addon compile avec et sans une Capability `optional`, dont le type rend
   l’absence explicite.
5. **Collision.** Deux Addons exposant la même Capability exigent un choix explicite.
6. **Contract invalide.** Une Capability présente dans deux clauses du même Contract est refusée.
7. **Cycle.** Une boucle de Capabilities est expliquée et refusée.
8. **Version.** Deux versions incompatibles du même Addon sont refusées avant compilation produit.
9. **Rust direct.** La System Definition est du code Rust compilable sans fichier source généré.

## Questions ouvertes

- Quelle forme Rust donne une Capability implémentable, documentable et directement utilisable?
- Quelle forme exprime un Addon Contract sans créer un langage parallèle à Rust?
- Comment remettre les imports typés à l’implémentation tout en partageant une Capability avec état?
- Comment une System Definition sélectionne-t-elle une implémentation lors d’une collision?
- Quelle part de Resolution appartient à des traits, à des constantes ou à une macro de compilation?
- Quel est le bootstrap minimal du futur gestionnaire d’Addons?

Ces questions doivent être décidées par des interfaces comparées et des scénarios compilables avant
la création du prochain crate produit.
