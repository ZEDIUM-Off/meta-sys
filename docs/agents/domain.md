# Domain Docs

How engineering skills should consume the domain documentation for this repository.

## Before exploring

- Read `CONTEXT.md` before naming or changing project concepts.
- Read `docs/vision.md` when work affects product scope, milestones, or positioning.
- Read `docs/composition.md` when work affects Addons, Capabilities, Contracts, Resolution,
  versioning, source management, or extension boundaries.
- Read relevant files under `docs/adr/` when that directory exists.

If one of these files does not exist, proceed silently.

## Layout

This repository uses a single-context layout:

```text
/
├── CONTEXT.md
├── docs/
│   ├── vision.md
│   ├── composition.md
│   ├── agents/
│   ├── references/
│   └── adr/              # created when the first ADR is warranted
└── src/
```

## Use canonical language

Use the terms defined in `CONTEXT.md` in source names, public APIs, tests, issues, and architectural documents.

If a required concept is absent or a term is overloaded, resolve the vocabulary before introducing a competing synonym.

## Preserve levels of certainty

`CONTEXT.md` contains canonical language.

`docs/vision.md` contains product direction.

`docs/composition.md` contains the current architectural direction and explicitly marks open
questions.

ADRs contain decisions that were hard to reverse, surprising without context, and selected through a genuine trade-off.

Do not silently promote a working hypothesis from `docs/composition.md` into a settled decision.

## Flag conflicts

If proposed work contradicts an existing ADR, identify the ADR and explain why reopening the decision may be warranted.
