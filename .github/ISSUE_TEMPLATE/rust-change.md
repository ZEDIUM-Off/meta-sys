---
name: Rust behavior change
about: Specify a Rust feature or bug fix before its TDD cycles
title: ""
labels: ""
assignees: ""
---

## Résultat observable

<!-- Décrire le comportement attendu du point de vue de l'appelant. -->

## Contexte et périmètre

- Documents de référence :
- Vocabulaire canonique concerné :
- Hors périmètre :

## Seams proposés

<!-- Pour chaque seam : module, interface, appelants, résultat observable et adaptateurs. -->

| Seam | Interface exposée | Ce qu'il cache | Pourquoi ce seam |
|---|---|---|---|
| | | | |

Confirmation explicite des seams :

## Contrats Rust

<!-- Une ligne par type ou opération exposée. Décrire le contrat, pas son implémentation. -->

| Item | Responsabilité | Entrées et contraintes | Retour garanti | Erreurs, panics, effets et invariants |
|---|---|---|---|---|
| | | | | |

## Matrice de scénarios

<!-- Chaque promesse du contrat doit être couverte. Les tests seront implémentés un par un. -->

| ID | Étant donné / Quand / Alors | Résultat observable indépendant | Type de test | Statut |
|---|---|---|---|---|
| S-01 | | | intégration / doctest / propriété / compile-fail | prévu |

## Critères de livraison

- [ ] Les seams sont confirmés avant le premier test.
- [ ] Chaque scénario possède une preuve RED → GREEN.
- [ ] Les rustdocs et doctests décrivent le contrat livré.
- [ ] `make check` passe.
