# task-159 — Nearest group · limited supply

A request is limited by available stock. Use the smaller count as value. Round value to the nearest multiple of 10. At the midpoint choose the higher multiple.

```rust
pub fn nearest_group_10(requested: u16, available: u16) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `nearest_group_10(65535, 4)` gives `0`.
- `nearest_group_10(13, 14)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-159/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-159`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
