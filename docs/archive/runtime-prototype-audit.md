# Audit historique du prototype runtime-first

> **Superseded.** Ce document prouve ce que le prototype runtime-first avait validé; il ne décrit
> plus la direction du produit. Son code et ses tests restent consultables dans l’historique Git au
> commit `71abdd1`. La direction actuelle est définie dans
> [`ADR-0001`](../adr/0001-static-composition-first.md) et
> [`docs/composition.md`](../composition.md).

## Verdict

Les quatorze tracer bullets #2 à #15 satisfont leurs critères d’acceptation. Le prototype est validé par `make check`: 46 tests Rust, rustfmt, Clippy avec warnings interdits, rustdoc et les règles structurelles Dylint. Le test unitaire de copie ABI passe également sous Miri.

Cet audit porte sur `main` au commit `8fa796e`, après les commits de décision et d’implémentation de
l’ABI native. Les issues GitHub constituent le journal RED → GREEN; les chemins cités sont
historiques et se lisent dans ce commit ou dans la dernière organisation du prototype au commit
`71abdd1`.

## #2 — Baseline Rust 1.98 et premier crate Kernel

- [x] `rust-toolchain.toml` fixe Rust 1.98.0 avec rustfmt et Clippy; `dylint.toml` et la configuration Dylint conservent leur nightly distinct.
- [x] `Cargo.toml` fixe `rust-version = "1.98"`; le commit de baseline `e1b66f2` contient un seul crate produit documenté, `meta-system-kernel`, héritant des lints workspace.
- [x] `Cargo.lock` est versionné. La fixture `cdylib` n’est apparue qu’en #15, lorsqu’un seam de build indépendant l’a justifiée.
- [x] La baseline n’exposait aucun comportement spéculatif; la responsabilité initiale est décrite dans `crates/meta-system-kernel/src/lib.rs`.
- [x] `Makefile`, `scripts/test-rust.sh`, `scripts/check-rust.sh` et `.github/workflows/rust-quality.yml` utilisent la toolchain et la gate communes.
- [x] `e1b66f2` précède tous les commits de comportement produit.

## #3 — Instance Pending sans fournisseur

- [x] `KernelRuntime::handle` accepte `KernelEvent::RegisterComponent` dans `src/runtime.rs` et `src/event.rs`.
- [x] `SystemGraph` distingue Definition, Instance, Requirement, Binding et Component Runtime dans `src/graph_view.rs`.
- [x] `necessary_requirement_without_provider_keeps_instance_pending` prouve Pending, l’absence de Binding et de Runtime dans `tests/pending_resolution.rs`.
- [x] `kernel_runtimes_keep_graph_state_isolated` prouve l’isolation de deux Runtimes.
- [x] Les tests passent uniquement par `KernelRuntime`; les commentaires RED/GREEN sont conservés sur l’issue #3.
- [x] Les rustdocs de `KernelRuntime::handle` documentent la garantie Pending et les erreurs.
- [x] La gate est verte; implémentation `c0b2b2e`.

## #4 — Activation par Capability compatible

- [x] `Capability` expose un `CapabilityContractId` et documente explicitement qu’il ne représente pas une permission.
- [x] `compatible_capability_binds_and_activates_pending_consumer` prouve l’unique Binding inspectable.
- [x] Le même test et `driver_start_failure_does_not_publish_active_runtime` imposent Active seulement après Bindings et démarrage du Runtime.
- [x] `SequentialExecutor` implémente le contrat interchangeable `EventLoopDriver` dans `src/driver.rs`.
- [x] `TransitionOutcome` expose transitions, Bindings créés et plan d’exécution sans exposer le Resolver.
- [x] Les scénarios RED/GREEN sont enregistrés sur #4 et testés via `KernelRuntime` dans `tests/capability_activation.rs`.
- [x] La gate est verte; implémentation `477f0f2`.

## #5 — Nettoyage et réactivation après retrait

- [x] `provider_removal_does_not_disturb_unrelated_binding` limite l’effet aux consumers liés au provider retiré.
- [x] Les tests de `tests/lifecycle_cleanup.rs` passent par `EventLoopDriver::stop`, y compris son échec explicite.
- [x] `active_component_can_record_owned_effect` et les assertions de retrait prouvent la disparition du Runtime et des Effects possédés.
- [x] `ResolutionState` ne contient que Pending et Active; `driver_stop_failure_is_not_a_resolution_state` garde l’échec dans `KernelError`.
- [x] `provider_removal_cleans_consumer_and_replacement_reactivates` prouve le nouveau Binding et le nouveau `ComponentRuntimeId`.
- [x] Toutes les observations utilisent outcomes et `SystemGraph`, jamais le stockage privé.
- [x] La gate est verte; implémentation `8688e30`.

## #6 — Facet typée dans un Context

- [x] `FacetSchema` et `Facet` sont des types publics distincts et inspectables.
- [x] le propriétaire `AddonId`, le `FacetTarget` et le Context sont observables.
- [x] `Context` exprime propriétaire, visibilité, parenté et cycle de vie, sans API de résolution de dépendance.
- [x] `tests/facet_context.rs` couvre Schema inconnu, cible inconnue, type de valeur et type de cible incompatibles.
- [x] `FacetValue` reste générique; aucune sémantique métier de Facet n’entre dans le Kernel.
- [x] Les six tests utilisent des `KernelEvent` et `SystemGraph`.
- [x] La gate est verte; implémentation `53ee9b4`.

## #7 — Sous-graphe affecté et travaux indépendants

- [x] `GraphState::affected_activation_plan` part de la mutation seed et calcule uniquement sa fermeture affectée.
- [x] `transitive_dependencies_produce_deterministic_ordered_fronts` prouve les fronts dépendants ordonnés.
- [x] `provider_mutation_places_independent_consumers_in_same_front` prouve le regroupement du travail indépendant.
- [x] `SequentialExecutor` fournit l’issue de référence déterministe.
- [x] `concurrent_driver_overlaps_independent_front_without_changing_outcome` mesure un chevauchement réel avec le même outcome.
- [x] `EventLoopDriver` n’expose ni mutex global, ni queue globale, ni affinité de thread.
- [x] Les trois tests restent au seam `KernelRuntime` dans `tests/affected_scheduling.rs`.
- [x] La gate est verte; implémentation `81c30d7`.

## #8 — Cycle Runtime et politiques de Binding

- [x] `KernelRuntime::handle` conserve la forme Current State + typed Event → Next State/outcome.
- [x] `add_binding_hook` trie par `(HookOrder, AddonId)`.
- [x] L’absence de hook conserve la sélection allow-all utilisée par les tests d’activation.
- [x] `policy_hook_rejects_binding_with_inspectable_reason` observe l’Addon, le Requirement et la raison.
- [x] `policy_hook_selects_another_compatible_provider` autorise l’influence uniquement parmi les candidates compatibles; la sélection hors contrat est rejetée.
- [x] Aucun type `Command` ni commit outbox n’existe dans l’interface.
- [x] `tests/binding_hooks.rs` exerce les hooks via `KernelRuntime`.
- [x] La gate est verte; implémentation `2fee9be`.

## #9 — Cycle Loader et Definition complète

- [x] `LoaderEvent` est l’unique stimulus public des transitions Loader.
- [x] `LoadRecord`, `LoadPhase`, `LoadTransition` et `LoadRejection` rendent phases et rejet inspectables.
- [x] `loader_rejects_events_that_bypass_required_phases` interdit tout saut; les hooks ultérieurs n’interviennent qu’à l’admission.
- [x] `ComponentMaterializer::inspect` retourne l’unique `ComponentDefinition` complète.
- [x] `failed_kernel_registration_never_reaches_registered_or_ready` prouve que Ready suit un enregistrement Kernel réussi.
- [x] `DeterministicMaterializer` ne contient aucune décision ABI native.
- [x] Les quatre tests de `tests/loader_lifecycle.rs` passent par Loader et observation Kernel.
- [x] La gate est verte; implémentation `f12da3b`.

## #10 — Politiques Loader selon activation

- [x] Sans hook, le parcours nominal atteint Admitted et Ready.
- [x] `add_hook` trie les hooks par `(HookOrder, AddonId)`; l’ordre est observé dans `active_loader_hooks_run_in_order_and_reject_inspectably`.
- [x] Un rejet produit `LoadPhase::Rejected` et un `LoadRejection` avec Addon et raison.
- [x] `hook_activation_never_replays_completed_admission` prouve l’absence de gouvernance rétroactive.
- [x] `one_addon_may_hold_loader_and_system_roles_without_correlation` emploie le même `AddonId` pour un hook Loader et une Facet Schema sans protocole Kernel.
- [x] `tests/loader_policies.rs` traverse les seams Loader et Kernel Runtime.
- [x] La gate est verte; implémentation `69658db`.

## #11 — Send FIFO et Send Receipt

- [x] `Room::accept` borne sa FIFO, attribue une `RoomSequence` monotone et retire exactement une étape de distribution.
- [x] `send_distributes_fifo_only_to_subscribed_mailboxes` livre uniquement au Runtime abonné.
- [x] `Mailbox` est bornée et stockée dans `ComponentRuntime`.
- [x] `SendReceipt` sépare acceptation Room, état de placement et processing observé par le Driver.
- [x] Les rustdocs et `bounded_mailbox_reports_full_without_retry` excluent transaction et retry.
- [x] `send_to_unavailable_logical_room_fails_explicitly` retourne `KernelError::UnavailableRoom`.
- [x] Les quatre tests de `tests/routing_send.rs` utilisent `KernelRuntime::send` et les outcomes publics.
- [x] La gate est verte; implémentation `7efff24`.

## #12 — Emit et broadcast sans ordre causal global

- [x] `emit_reaches_only_subscribers_declared_by_the_source_contract` limite emit aux routes typées du `RoutingContract` source.
- [x] `broadcast_reaches_only_registered_active_listeners` limite broadcast aux listeners actifs enregistrés.
- [x] `KernelRuntime::send`, `emit` et `broadcast` transportent tous le même type `Event`.
- [x] Chaque emit passe encore par `Room::accept` et sa séquence FIFO locale.
- [x] `independent_emit_rooms_form_one_concurrent_driver_front` fait progresser deux Rooms dans un même front et mesure deux traitements simultanés sans ordre global exposé.
- [x] L’ordre local dépendant reste imposé par les fronts testés en #7.
- [x] `tests/routing_publish.rs` emploie le seam Kernel Runtime et un Driver concurrent.
- [x] La gate est verte; implémentation `144f6ab`.

## #13 — Overflow sans retry implicite

- [x] `MailboxPolicy` contient une `QueueCapacity` positive explicite et `Mailbox` l’impose.
- [x] La Mailbox invoque la stratégie Component `RejectNew` ou `DropOldest` à saturation.
- [x] `DeliveryReceipt` conserve l’état `RejectedFull` ou `DeliveredAfterDroppingOldest` séparément de processing.
- [x] Aucun mécanisme de retry, transaction ou backpressure globale n’est présent.
- [x] `docs/kernel.md` impose que toute remise en file ou tout renvoi repasse par send, emit ou broadcast.
- [x] `tests/routing_overflow.rs` couvre les deux outcomes sélectionnés via `KernelRuntime::send`.
- [x] `docs/kernel.md` fixe `RejectNew` comme défaut prototype et documente `DropOldest`.
- [x] La gate est verte; implémentation `9de887b`.

## #14 — Adresse logique pendant réactivation

- [x] `deactivate_routing` retire les Rooms du propriétaire; le test observe l’échec explicite du send inactif.
- [x] la désactivation retire `RoomRuntimeId`, Component Runtime et sa Mailbox.
- [x] la valeur `RoomAddress` existante est conservée comme référence logique.
- [x] après fournisseur de remplacement, cette même adresse résout un nouveau `RoomRuntimeId`.
- [x] la nouvelle Mailbox est vide puis ne contient que l’Event post-réactivation.
- [x] `tests/routing_reactivation.rs` couvre le cycle complet par `KernelRuntime`.
- [x] La gate est verte; contrat verrouillé par `f51bfc0`.

## #15 — Materializer natif de confiance

- [x] La décision `docs/development/native-component-abi.md` est commitée en `73e56c5`, avant l’implémentation `8fa796e`.
- [x] `NativeMaterializer::materialize` ouvre et conserve la bibliothèque à Materialized; `inspect` ne résout et copie la Definition qu’à Inspected.
- [x] le descripteur ABI ID/Requirements/Capabilities devient la même `ComponentDefinition` complète que celle remise par l’adaptateur déterministe.
- [x] le chemin filesystem est canonisé dans le materializer privé et aucune Capability filesystem n’est créée.
- [x] la documentation ABI et `docs/kernel.md` disent explicitement que Requirements et Capabilities ne forment pas une sandbox.
- [x] l’exception `unsafe_code` est limitée au module natif; chaque bloc documente son invariant. Miri valide `static_descriptor_is_copied_into_complete_definition`; la tentative native atteint la limite Miri explicite « `dlopen` unsupported », puis le test hôte réel passe.
- [x] `fixtures/native-component` définit indépendamment l’ABI C et `tests/native_loader.rs` parcourt Declared → Ready en observant ID, Requirement et Capability dans le Kernel.
- [x] La gate est verte; décision `73e56c5`, implémentation `8fa796e`.

## Gate finale

Commandes exécutées sur le worktree final:

```text
make check
cargo +nightly-2026-05-28-aarch64-unknown-linux-gnu miri test -p meta-system-kernel --lib
```

Résultats:

- 46 tests Rust verts sur les deux crates workspace;
- test Miri de copie ABI vert;
- rustfmt, Clippy `-D warnings`, rustdoc et Dylint verts;
- limites Dylint respectées: 50 lignes par fonction, 12 fonctions par fichier, 400 lignes par fichier et 5 arguments;
- toutes les issues #2 à #15 closes;
- aucun critère d’acceptation non couvert identifié.
