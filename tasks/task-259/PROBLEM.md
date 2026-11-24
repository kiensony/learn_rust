# task-259 — Number of divisors · limited supply

A request is limited by available stock. Use the smaller count as value. Count positive numbers from 1 through 10 that divide value evenly. Every positive number divides zero.

```rust
pub fn number_of_divisors_10(requested: u8, available: u8) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `number_of_divisors_10(255, 4)` gives `3`.
- `number_of_divisors_10(13, 14)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-259/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-259`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
