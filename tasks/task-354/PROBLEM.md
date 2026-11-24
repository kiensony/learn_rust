# task-354 — Observe each guard leaving scope

Create one supplied Visit guard per visit. Return the final number recorded by its destructor after all guards leave scope. Use Drop to count closings. Do not set the counter manually.

```rust
pub fn closed_visits(visits: u8) -> u8
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `closed_visits(2)` gives `2`.
- `closed_visits(3)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-354/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-354`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
