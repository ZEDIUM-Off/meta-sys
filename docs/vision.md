# Vision du projet

## Thèse

Le dépôt ne devrait pas rester l’unité fondamentale du logiciel. Un System doit être une composition vivante, portable et indépendante de la topologie qui l’exécute; le code n’en est qu’une contribution.

Meta-system est un langage et orchestrateur minuscule et augmentable d’évolutions d’état explicites et calculées. Presque tout comportement du framework doit pouvoir se comprendre comme une machine à états réagissant à des Events:

```text
Current State + Event → Next State
```

Chaque Component est une machine à états. Le défi produit central est de résoudre et d’orchestrer efficacement l’ensemble de leurs états et relations, tout en offrant des interfaces intuitives aux développeurs.

## Positionnement

Le Kernel est délibérément petit, neutre vis-à-vis des domaines et purement orchestratorial. Il ne possède seul aucune Capability métier ou tournée vers le monde extérieur. Les Addons augmentent progressivement cette base avec les capacités, politiques et expériences nécessaires, sans transformer leurs domaines en concepts du Kernel.

La simplicité et l’optimisation sont des objectifs simultanés. Le modèle doit rendre l’exécution parallèle structurellement possible dès l’origine lorsque les dépendances le permettent, et préserver un comportement local compréhensible et déterministe lorsque l’ordre importe.

Le graphe vivant, plutôt que le dépôt ou le déploiement, est l’objet de composition. Les dépendances, Requirements, Capabilities et Bindings restent explicites afin que l’évolution du System soit inspectable et explicable. Cette explicitation facilite la composition; elle ne remplace ni la compréhension du modèle par les développeurs ni la conception correcte des Addons.

La base fondatrice doit rester open source. La portabilité appartient au modèle, pas à une offre privilégiée.

## Composition par Addons

**Addon** est le terme canonique. Un Loader Addon augmente le chargement, un System Addon contribue au System et un Runner Addon augmente l’exécution ou la supervision. Une même unité peut cumuler ces rôles ou augmenter les machines à états introduites par d’autres Addons.

Le même Kernel peut composer, par exemple:

- une application de type IDE avec des Addons d’interface utilisateur, de filesystem et de Git;
- une application de type navigateur avec des Addons d’interface utilisateur, de navigation et de réseau;
- une application unique réunissant ces compositions.

Ces exemples ne sont ni des primitives du Kernel ni une feuille de route imposée. Une application possède un Runtime. Des applications autonomes ont donc des Runtimes distincts; leur communication pourra éventuellement être apportée par des Addons, sans faire d’un maillage distribué une primitive du Kernel.

## Chargement et confiance

Le chargement dynamique est précieux et peut suivre une approche proche de Cordis. Du code natif exécuté dans le processus est toutefois réputé de confiance par le fait même de son intégration: un Requirement exprime un besoin de composition, jamais une autorisation ni une frontière de sécurité.

Permissions, politiques, sandbox, distribution, stockage et autres capacités produit relèvent des Addons ou des choix de l’intégrateur. Meta-system préfère une petite base progressivement augmentée à un système abstrait de vérification ou à des primitives spéculatives.

## Relation à Cordis

Cordis et `cordis-rs` sont des références pragmatiques, non l’architecture cible. Meta-system en retient les idées qui servent son modèle minimal, notamment la résolution réactive, les Effects possédés par un cycle de vie, les Component Instances vivantes et la distinction `Pending`/`Active`.

## Non-objectifs

- Faire du Kernel une plateforme métier, une bibliothèque standard ou un catalogue de capacités.
- Faire des permissions, de la sandbox ou d’un maillage inter-Runtime des primitives du Kernel.
- Figer maintenant les futurs Addons, produits ou modes de distribution.

## Direction proche

Le travail immédiat consiste à valider le plus petit modèle cohérent d’état, de résolution et d’Events, ainsi que sa capacité à orchestrer efficacement des Components indépendants. L’implémentation Rust sert cette validation sans figer prématurément l’API publique ni la forme des futurs Addons.
