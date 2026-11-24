# task-108 — Two counters · limited supply

A request is limited by available stock. Use the smaller count as value. One dial counts in groups of 10. Return both the complete groups and the remainder.

```rust
pub fn two_counters_10(requested: u16, available: u16) -> (u16, u16)
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `two_counters_10(65535, 4)` gives `(0, 4,)`.
- `two_counters_10(13, 14)` gives `(1, 3,)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-108/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-108`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
