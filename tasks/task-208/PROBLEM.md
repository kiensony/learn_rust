# task-208 — Priority fee · limited supply

A request is limited by available stock. Use the smaller count as value. Return 0 for zero, 10 for a positive value through 12, and twice 10 otherwise.

```rust
pub fn priority_fee_10(requested: u16, available: u16) -> u8
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `priority_fee_10(65535, 4)` gives `10`.
- `priority_fee_10(13, 14)` gives `20`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-208/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-208`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
