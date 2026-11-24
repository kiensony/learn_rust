# task-256 — Find the first triangular total

Starting with zero, add 1, then 2, then 3 and so on. Return how many additions it takes to reach or exceed goal. Zero already reaches a goal of zero. The total may cross the u16 limit.

```rust
pub fn steps_to(goal: u16) -> u16
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `steps_to(3)` gives `2`.
- `steps_to(5)` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-256/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-256`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
