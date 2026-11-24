# task-209 — Status lamp · limited supply

A request is limited by available stock. Use the smaller count as value. Return "low" below 10, "full" above 12, and "ready" in between (including the boundaries).

```rust
pub fn status_lamp_10(requested: u16, available: u16) -> &'static str
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `status_lamp_10(65535, 4)` gives `"low"`.
- `status_lamp_10(13, 14)` gives `"full"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-209/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-209`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
