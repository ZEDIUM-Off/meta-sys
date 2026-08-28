# Native Component ABI v1

## Décision

Le prototype charge des Components natifs de confiance dans le processus hôte avec une frontière `extern "C"` versionnée. La bibliothèque exporte exactement le symbole non manglé suivant:

```c
NativeComponentDescriptor meta_system_component_v1(void);
```

`NativeComponentDescriptor` utilise `repr(C)` côté Rust et son équivalent C. Il contient:

- `abi_version: uint32_t`, égal à `1`;
- `reserved: uint32_t`, égal à `0` dans cette version;
- `definition_id: uint64_t`;
- une vue immuable `(pointer, size_t)` de Requirements;
- une vue immuable `(pointer, size_t)` de Capabilities.

Chaque Requirement contient un identifiant `uint64_t` et un identifiant de Capability Contract `uint64_t`. Chaque Capability suit la même forme. Les scalaires ne transportent ni allocation Rust, ni trait object, ni référence Rust, ni enum dont la représentation serait implicite.

Cette ABI suppose que l’hôte et la bibliothèque ciblent la même plateforme, la même architecture et la même convention C. Toute évolution incompatible reçoit une nouvelle version et un nouveau symbole; la v1 n’est jamais étendue en réinterprétant ses champs réservés silencieusement.

## Ordre Loader

Le `NativeMaterializer` respecte les phases existantes:

1. `Located` résout et vérifie le chemin de bootstrap;
2. `Materialized` ouvre la bibliothèque et conserve son handle;
3. `Inspected` résout le symbole, appelle le point d’entrée, valide le descripteur et copie toutes les contributions dans une `ComponentDefinition` Rust complète;
4. les politiques Loader évaluent cette définition complète avant son enregistrement par le Kernel Runtime.

Le descripteur FFI est un format de transfert privé. Il n’est jamais inséré dans le System Graph et ne concurrence pas `ComponentDefinition`.

## Invariants de sûreté

Le chargement natif est réservé à des bibliothèques de confiance. L’intégrateur garantit que:

- les routines d’initialisation et de terminaison de la bibliothèque peuvent s’exécuter dans le processus hôte;
- `meta_system_component_v1` possède exactement la signature publiée et ne déroule jamais une panique à travers la frontière C;
- tout pointeur non nul est correctement aligné et lisible pour le nombre d’éléments annoncé;
- un pointeur nul annonce une longueur nulle;
- les tableaux pointés restent immuables et valides tant que le handle de bibliothèque est chargé;
- la bibliothèque ne conserve aucune référence vers une donnée temporaire appartenant à l’hôte.

Le code `unsafe` de l’hôte se limite à l’ouverture, à la résolution typée du symbole, à son appel et à la création temporaire des slices validées. Le handle reste possédé par le `NativeMaterializer`, donc aucun symbole ou pointeur n’est utilisé après déchargement.

## Limite de confiance

Le chemin de bibliothèque est une donnée de bootstrap interprétée uniquement par le Loader. Son usage du filesystem n’introduit aucune Capability filesystem dans une Component Definition.

Une bibliothèque native s’exécute dans le processus et peut employer `std`, FFI ou des appels système en dehors des Capabilities déclarées. Les Requirements décrivent la composition; ils ne constituent pas une sandbox, une permission ou une isolation. Les politiques de provenance, de signature, de reconstruction, de confinement ou d’exécution hors processus restent la responsabilité de l’intégrateur ou de futurs Addons.

## Vérification

Une fixture `cdylib` exporte l’ABI indépendamment des types Rust de l’hôte. Un test d’intégration parcourt le Loader complet jusqu’à `Ready` et observe la `ComponentDefinition`, ses Requirements et ses Capabilities dans le Kernel Runtime. La gate standard exécute rustfmt, Clippy, les tests, rustdoc et Dylint. Un test unitaire fait vérifier par Miri la validation et la copie des vues ABI à partir de tableaux statiques. Miri ne sait pas interpréter `dlopen` ni l’appel FFI dynamique; le parcours natif réel s’appuie donc sur la gate hôte et les invariants explicités ci-dessus.
