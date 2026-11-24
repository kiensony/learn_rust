# task-207 — Three shelves · limited supply

A request is limited by available stock. Use the smaller count as value. Return 0 below 10, 1 from 10 through 12 inclusive, and 2 above 12.

```rust
pub fn three_shelves_10(requested: u16, available: u16) -> u8
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `three_shelves_10(65535, 4)` gives `0`.
- `three_shelves_10(13, 14)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-207/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-207`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
