# Qualité du développement Rust

Le code Rust de Meta-System suit une philosophie de _norme outillée_ : les modules restent
petits, documentés et compréhensibles en un scan, et toute exception est locale, motivée et
visible pendant la revue.

Les fonctionnalités et corrections comportementales suivent le
[protocole TDD Rust](rust-tdd.md) avant d'atteindre ce gate de qualité.

## Gate unique

Exécuter avant de livrer une modification Rust :

```bash
make check
```

Le `Makefile` constitue l'interface stable des commandes du projet ; `scripts/check-rust.sh`
implémente actuellement ce gate. Il vérifie le formatage, la compilation, Clippy, les tests,
rustdoc et la politique Dylint. Tant que le workspace ne contient aucun crate produit, il teste la
bibliothèque Dylint seule.

L’environnement de développement requiert :

```bash
rustup component add clippy rustfmt
cargo install --locked cargo-dylint dylint-link
```

La bibliothèque Dylint possède son propre nightly dans
`tools/dylint/meta_sys_style/rust-toolchain.toml`. Elle reste isolée du toolchain des crates du
produit, car elle compile contre les API internes et instables de `rustc`.

## Lisibilité en un scan

Chaque crate active les lints du workspace :

```toml
[lints]
workspace = true
```

Les règles exigent notamment :

- une documentation rustdoc pour les items publics et privés ;
- des sections `# Errors`, `# Panics` et `# Safety` lorsqu’elles s’appliquent ;
- des liens rustdoc valides et, autant que possible, des exemples exécutables ;
- du code `unsafe` refusé par défaut, limité à un seam FFI motivé et accompagné de ses invariants
  `# Safety` et `// SAFETY:` ;
- une largeur formatée de 100 colonnes ;
- au plus 50 lignes et 5 paramètres par fonction ;
- au plus 12 fonctions et 400 lignes par fichier source ;
- une complexité cognitive maximale de 15 et quatre niveaux d’imbrication.

La documentation explique le contrat, les invariants, les unités et les raisons non évidentes.
Elle ne paraphrase pas la syntaxe. Un fichier commence par `//!` pour annoncer sa responsabilité et
ses invariants ; un lecteur doit pouvoir décider où approfondir sans lire chaque corps de fonction.

## Exceptions

Une limite signale d’abord une responsabilité trop large. Extraire un type, un module ou une étape
nommée avant de demander une exception. Lorsqu’une forme longue est réellement plus claire, employer
`#[expect(..., reason = "...")]` au niveau le plus étroit et expliquer la contrainte qui justifie
l’écart. Les désactivations globales et les raisons vagues ne constituent pas une politique.

Les seuils sont définis dans `clippy.toml` et `dylint.toml`. Une évolution de seuil modifie ces
sources exécutables et ajoute dans la même modification une fixture ou un cas de test qui démontre
la nouvelle frontière.

## Skill Rust partagé

Le skill global `/home/zedium/.agents/skills/rust-skills` est la référence commune à Meta-System et
Zedflow. Il faut charger son index avant une écriture, une revue ou une refactorisation Rust, puis
lire les fichiers `rules/` pertinents pour les catégories réellement touchées. Les règles
automatiques restent l’autorité lorsqu’un conseil général du skill entre en conflit avec la
politique locale.
