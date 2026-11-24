# task-156 — Saturating debit · limited supply

A request is limited by available stock. Use the smaller count as value. Remove 10 units from value. Stop at zero.

```rust
pub fn saturating_debit_10(requested: u16, available: u16) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `saturating_debit_10(65535, 4)` gives `0`.
- `saturating_debit_10(13, 14)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-156/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-156`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
