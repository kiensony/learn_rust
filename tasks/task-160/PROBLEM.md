# task-160 — Fractional fee · limited supply

A request is limited by available stock. Use the smaller count as value. Charge 10 percent of value. Discard any fraction of a cent.

```rust
pub fn fractional_fee_10(requested: u16, available: u16) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `fractional_fee_10(65535, 4)` gives `0`.
- `fractional_fee_10(13, 14)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-160/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-160`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
