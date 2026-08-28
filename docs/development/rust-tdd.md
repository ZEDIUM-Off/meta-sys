# Protocole TDD Rust

Toute fonctionnalité ou correction qui modifie un comportement observable suit ce protocole. Le
but n'est pas de maximiser le nombre de tests, mais de construire des modules profonds dont le
contrat reste stable, compréhensible et vérifiable depuis un seam convenu.

Le flux est : **contrat → seam confirmé → scénario → RED → GREEN**, une tranche verticale à la
fois. La refactorisation vient ensuite, pendant la revue.

## 1. Cadrer le changement

1. Lire l'issue ou la spécification et les documents déclenchés par `AGENTS.md`. Employer le
   vocabulaire canonique de `CONTEXT.md` dans les interfaces et les noms de tests.
2. Charger les skills `tdd`, `codebase-design` et `rust-skills`, puis les règles Rust liées aux
   types, erreurs, tests et documentations réellement touchés.
3. Remplir les sections du modèle `.github/ISSUE_TEMPLATE/rust-change.md`. Si le changement ne
   possède pas encore d'issue, produire les mêmes informations dans la tâche sans créer d'issue
   sans autorisation.
4. Proposer les seams à tester et demander leur confirmation explicite au responsable du
   changement. Le silence ne vaut pas confirmation.

Cette phase est terminée lorsque le résultat observable, le hors-périmètre, les seams et leurs
interfaces sont écrits, et que les seams sont confirmés. Aucun test n'est écrit avant ce gate.

## 2. Concevoir le contrat avant l'implémentation

En Rust, une interface n'est pas un fichier d'en-tête et une classe n'est pas la forme par défaut.
Choisir la forme qui exprime le domaine : `struct`, `enum`, newtype, `trait` ou fonction. Une
interface comprend sa signature, ses invariants, ses erreurs, ses effets et ses contraintes
d'usage.

Concevoir l'ensemble du contrat dans l'issue pour détecter les incohérences, puis ne matérialiser
dans le code que les types et signatures nécessaires à la prochaine tranche. Une fonction possède
une responsabilité nommable ; si sa description contient deux buts indépendants, séparer le
contrat avant d'écrire le test.

Chaque module commence par `//!` et décrit sa responsabilité, ses invariants et ce qu'il cache.
Chaque type, fonction et méthode, publique ou privée, porte une rustdoc qui précise, lorsque cela a
du sens :

- le but et le résultat observable de l'opération ;
- la signification de chaque entrée, ses unités, contraintes, ownership et durées de vie utiles ;
- la valeur retournée et ce qu'elle garantit ;
- les erreurs, panics et préconditions de sûreté dans `# Errors`, `# Panics` et `# Safety` ;
- les effets, transitions d'état, règles d'ordre, d'idempotence ou d'annulation ;
- un exemple exécutable pour l'usage principal.

La documentation décrit le contrat et ses raisons, pas la syntaxe. La documentation privée peut
être plus courte, mais elle nomme toujours la responsabilité, les contraintes non évidentes des
entrées et la garantie de sortie. Ces détails restent couverts depuis le seam confirmé : une
fonction privée ne reçoit pas automatiquement un test couplé à son implémentation.

Si Rust exige un corps avant le premier test, la tranche active peut employer temporairement
`todo!("RED: comportement attendu")`. `todo!` et `unimplemented!` sont interdits par le gate final.
Un `trait` n'est introduit que si le comportement varie réellement ; un faux de test et un
adaptateur de production peuvent établir cette variation, mais un trait ne sert pas uniquement à
espionner une implémentation interne.

Cette phase est terminée lorsque la prochaine tranche compile assez pour accueillir un test et que
son contrat ne contient plus d'ambiguïté connue.

## 3. Inventorier les scénarios

Avant de coder les tests, remplir la matrice de scénarios de l'issue. Chaque promesse de la rustdoc
doit pointer vers au moins un scénario. Examiner selon le contrat :

- exemple nominal et exemples alternatifs significatifs ;
- valeurs limites, entrées vides, minimales, maximales et invalides ;
- chaque variante d'erreur ou panic documentée ;
- invariants, transitions d'état et propriété d'idempotence ou de round-trip ;
- ordering, backpressure, annulation et concurrence lorsqu'ils sont observables ;
- ownership des Effects et comportement de nettoyage lorsqu'un cycle de vie est impliqué.

Les résultats attendus proviennent de la spécification, d'un exemple calculé ou d'un invariant
indépendant. Ils ne recomputent pas l'algorithme de l'implémentation. La matrice peut être complète
avant le code ; les tests, eux, sont écrits un par un.

## 4. Exécuter une tranche RED → GREEN

### RED

1. Choisir une seule ligne non couverte de la matrice.
2. Écrire un test qui observe ce comportement depuis le seam confirmé.
3. Donner au test un nom de scénario et une structure Arrange → Act → Assert lisible.
4. Exécuter uniquement ce test :

   ```bash
   make test-one PACKAGE=<crate> TEST=<module::nom_exact_du_test>
   ```

   Pour un doctest ou un contrat `compile_fail`, exécuter les doctests du crate :

   ```bash
   make test-doc PACKAGE=<crate>
   ```

5. Vérifier que le test existe bien, s'exécute et échoue pour l'absence précise du comportement.
   Une erreur d'environnement, de compilation sans rapport ou de fixture n'est pas un RED valide.
6. Conserver dans la PR la commande et la raison de l'échec observé.

### GREEN

1. Implémenter le minimum nécessaire pour satisfaire ce scénario, sans anticiper les suivants.
2. Relancer exactement la même commande jusqu'à ce qu'elle passe.
3. Exécuter ensuite tous les tests du crate :

   ```bash
   make test-package PACKAGE=<crate>
   ```

4. Mettre à jour la matrice et sélectionner la tranche suivante.

Le cycle reste RED → GREEN. Renommages, extractions et généralisations attendent la revue afin que
chaque cycle mesure une seule évolution de comportement.

## 5. Choisir le bon niveau de test

- Un seam externe se teste depuis `tests/` à travers les exports publics du crate.
- Un seam interne explicitement confirmé peut se tester dans un module `#[cfg(test)]`, à travers
  son interface `pub(crate)` ou `pub(super)`, jamais en appelant ses détails privés.
- Un exemple d'utilisation vit de préférence dans la rustdoc et s'exécute comme doctest.
- Un système externe, le temps, l'aléatoire ou le filesystem se remplace au seam par un adaptateur
  déterministe. Préférer un fake comportemental ; un mock vérifie le résultat observable, pas les
  appels internes.
- Un test couvre un comportement logique. Plusieurs assertions sont acceptables lorsqu'elles
  décrivent ensemble ce seul résultat observable.

Les outils spécialisés sont ajoutés quand le risque existe :

| Risque | Outil ou forme de test |
|---|---|
| Invariants sur un grand espace d'entrées | `proptest` avec stratégies du domaine |
| Parseur ou sérialisation | exemples connus et propriété de round-trip |
| Contrat de compilation | doctest `compile_fail` ou `trybuild` |
| Concurrence et interleavings | temps déterministe puis `loom` pour les primitives |
| Code `unsafe` exceptionnellement autorisé | invariants `# Safety` et `cargo miri test` |
| Logique critique difficile à distinguer | `cargo-mutants` sur le crate ciblé |
| Recherche de zones non exercées | `cargo-llvm-cov`, comme diagnostic et non comme preuve |

Ces dépendances ne sont pas ajoutées par anticipation : le contrat ou le risque doit justifier
leur coût.

## 6. Revue, refactorisation et livraison

Quand toute la matrice est GREEN :

1. Refactoriser par petits pas sans changer les contrats ni les résultats observables ; relancer
   les tests du crate après chaque pas.
2. Utiliser le skill `code-review` avec un fixed point disponible pour séparer conformité aux
   standards et conformité à la spécification.
3. Vérifier que chaque ligne de la matrice possède un test et que chaque test pointe vers une
   promesse du contrat. Supprimer les tests tautologiques ou couplés à l'implémentation.
4. Exécuter le gate final, qui inclut toutes les suites de tests :

   ```bash
   make check
   ```

5. Compléter `.github/pull_request_template.md` avec le lien vers la confirmation des seams et les
   preuves RED → GREEN.

Une modification est livrable lorsque les contrats et rustdocs sont à jour, tous les scénarios
sont GREEN, aucun placeholder ne subsiste, `make check` passe et la preuve de processus permet à un
reviewer de relier spécification, test et comportement.
