## Project context

- **Domain language or meta-model changes**: read `CONTEXT.md` and preserve its canonical vocabulary.
- **Product scope or positioning**: read `docs/vision.md` before proposing features, milestones, or product architecture.
- **Kernel architecture**: read `docs/kernel.md` before changing kernel primitives, lifecycle, dependency resolution, effects, facets, or extension boundaries.
- **Conversation provenance**: read `docs/references/project-conversations.md` when the origin or certainty of a project assumption matters.
- **Rust features and bug fixes**: load `tdd`, `codebase-design`, and `rust-skills`, then follow `docs/development/rust-tdd.md`. Record the proposed seams and obtain user confirmation before writing the first test; implement one RED → GREEN slice at a time.
- **Rust implementation or review**: load the global `rust-skills` skill, read the relevant linked rules, then follow `docs/development/rust-quality.md`. New crates inherit `[workspace.lints]`; substantive Rust changes pass `make check`.

## Agent skills

### Issue tracker

Issues and specs are tracked in this repository's GitHub Issues. See `docs/agents/issue-tracker.md`.

### Triage labels

Triage uses the five default canonical labels. See `docs/agents/triage-labels.md`.

### Domain docs

This repository uses a single-context domain documentation layout. See `docs/agents/domain.md`.
