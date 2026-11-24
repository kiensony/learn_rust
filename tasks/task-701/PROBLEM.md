# task-701 — Interpret a directional command

Home resets position to zero. Stay preserves it. Shift applies the signed offset, clamping at the i16 limits.

```rust
pub fn position(command: Motion, at: i16) -> i16
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `position(Motion::Home, 9)` gives `0`.
- `position(Motion::Shift(-4), 2)` gives `-2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-701/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-701`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
