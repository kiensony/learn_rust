# task-255 — Digit count · limited supply

A request is limited by available stock. Use the smaller count as value. Count the base-10 digits of value. Zero takes one digit.

```rust
pub fn digit_count_10(requested: u8, available: u8) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `digit_count_10(255, 4)` gives `1`.
- `digit_count_10(13, 14)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-255/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-255`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
