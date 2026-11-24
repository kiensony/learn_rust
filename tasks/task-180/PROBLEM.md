# task-180 — Admission gate · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Admit values at least 5.

```rust
pub fn admission_gate_5(issued: u16, returned: u16) -> bool
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `admission_gate_5(4, 0)` gives `false`.
- `admission_gate_5(13, 1)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-180/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-180`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
