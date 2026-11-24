# task-258 — Countdown checksum · limited supply

A request is limited by available stock. Use the smaller count as value. Start at value and repeatedly subtract 10 while positive. Sum the positive numbers visited.

```rust
pub fn countdown_checksum_10(requested: u8, available: u8) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `countdown_checksum_10(255, 4)` gives `4`.
- `countdown_checksum_10(13, 14)` gives `16`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-258/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-258`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
