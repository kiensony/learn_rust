# task-257 — Cycle ticks · limited supply

A request is limited by available stock. Use the smaller count as value. A wheel starts at position zero on a ring of 12 positions. Advance 10 places value times and return its final position.

```rust
pub fn cycle_ticks_10(requested: u8, available: u8) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `cycle_ticks_10(255, 4)` gives `4`.
- `cycle_ticks_10(13, 14)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-257/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-257`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
