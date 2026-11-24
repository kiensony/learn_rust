# task-109 — A full tray · limited supply

A request is limited by available stock. Use the smaller count as value. Return true exactly when value tiles fill a whole number of trays of 10, including zero trays.

```rust
pub fn a_full_tray_10(requested: u16, available: u16) -> bool
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `a_full_tray_10(65535, 4)` gives `false`.
- `a_full_tray_10(65535, 0)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-109/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-109`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
