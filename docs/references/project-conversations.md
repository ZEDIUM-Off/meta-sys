# Project Conversations

These ChatGPT conversations record the exploration that led to the project direction and to the
runtime-first prototype that was later superseded.

They are provenance, not specifications. When they conflict with repository documentation,
`CONTEXT.md`, accepted ADRs, and the current code take precedence.

## Primary kernel discussions

- [Expliquer Cordis et meta system](chatgpt-conversation://6a8bf4ef-b7f4-83ed-adfe-645fa0f55ba4) — Cordis fundamentals, reversible effects, reactive dependencies, semantic Facets, recursive Extensions, and the decision to build a new Rust kernel inspired by Cordis.
- [Branche · Expliquer Cordis et meta system](chatgpt-conversation://6a8c2aff-8224-83eb-9af5-314d988341ca) — extended architecture and naming exploration; useful for the progression from distributed runtime to domain-neutral kernel.
- [Esquisser le kernel](chatgpt-conversation://6a8c3511-93c0-83eb-9188-f2b44b422684) — latest kernel boundary, bootstrap model, definition/runtime separation, explicit Bindings, Facet provenance, and the principle that the Kernel has no external authority.

## Product direction

- [Réflexion sur le metasystème](chatgpt-conversation://6a830cfc-9ae8-83eb-88aa-18e965291830) — product thesis: the System rather than the repository as the primary software object; open software fabric and initial agentic-runtime wedge.

## Earlier browser and framework exploration

- [Navigateur Chromium en Rust](chatgpt-conversation://6a81c471-be10-83eb-89ef-372d263edf2d) — programmable browser origin, extension model, distributed execution, permissions, and verifiable authority ideas.
- [Branche · Navigateur Chromium en Rust — framework](chatgpt-conversation://6a830168-8520-83ed-a304-1e0ab1c56873) — comparison with wasmCloud, Fuchsia, Theia, Extism, Dapr, Erlang/OTP, Tauri, Ray, and Urbit; transition from browser framework to malleable meta-system.
- [Branche · Navigateur Chromium en Rust — UI](chatgpt-conversation://6a830102-1ddc-83eb-8df9-7647c3136834) — GPUI, native clients, rendering surfaces, and the separation between UI contracts and a specific renderer.

## Reading guidance

The early discussions correctly identify recursive `provides`/`requires` composition and reusable
Capabilities as the central value. Their later promotion of Components, Events, a living graph,
dynamic loading and distribution into universal Kernel primitives is superseded by
[`ADR-0001`](../adr/0001-static-composition-first.md).

Read [`docs/vision.md`](../vision.md) and [`docs/composition.md`](../composition.md) for the current
static-first direction. Use the conversations only to understand how the trade-off was explored.

In particular, `Mesh` and `Node` remain possible Capabilities introduced by Addons; they are not
Meta-system primitives. `Extension`, `Component`, `Provider`, `Requirement` and `Binding` are not
current canonical terms.

Naming explorations such as Zeta, Valence, Coval, Metis, Zetis, and Mallea did not produce a final public name. Use `Meta-system` as the conceptual term and `meta-sys` as the repository name until a naming decision is recorded.
