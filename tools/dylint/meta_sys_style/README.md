# `meta_sys_style`

### What it does

Enforces the scan-friendly structural thresholds configured in the linted workspace's
`dylint.toml`: function lines, function arguments, functions per source file, and source-file
lines.

### Why is this bad?

Large units hide responsibilities and invariants. A strict, explicit boundary makes decomposition
part of ordinary development instead of a subjective review request.

### Example

```rust
fn activate(a: A, b: B, c: C, d: D, e: E, f: F) {}
```

Use instead:

```rust
struct Activation { /* named inputs */ }

fn activate(input: Activation) {}
```
