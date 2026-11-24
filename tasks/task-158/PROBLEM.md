# task-158 — Digit place · limited supply

A request is limited by available stock. Use the smaller count as value. Read the base-10 digit in the second position from the right.

```rust
pub fn digit_place_10(requested: u16, available: u16) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `digit_place_10(65535, 4)` gives `0`.
- `digit_place_10(13, 14)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-158/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-158`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
