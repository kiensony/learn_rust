# task-206 — Authorize library entry

A closed library admits nobody. When open, it admits members and invited guests. Return whether entry is permitted.

```rust
pub fn admitted(member: bool, invited: bool, closed: bool) -> bool
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `admitted(true, false, false)` gives `true`.
- `admitted(true, true, true)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-206/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-206`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
