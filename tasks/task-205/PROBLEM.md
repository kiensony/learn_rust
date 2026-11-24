# task-205 — Service interval · limited supply

A request is limited by available stock. Use the smaller count as value. Require a positive multiple of 10.

```rust
pub fn service_interval_10(requested: u16, available: u16) -> bool
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `service_interval_10(65535, 4)` gives `false`.
- `service_interval_10(10, 11)` gives `true`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-205/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-205`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
