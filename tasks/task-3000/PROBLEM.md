# task-3000 — Transfer assessment · account for overlapping incidents

An incident timeline uses half-open intervals [start,end) in u16 ticks. Some incidents overlap, touch, repeat or have zero duration. Compute total covered ticks as u32. Ignore reversed intervals. Input order is arbitrary. The starter double counts overlap. Find a counterexample distinct from the public examples, write a regression against a tick-by-tick oracle on a small domain, and explain the time and memory tradeoff of your production algorithm.

```rust
pub fn affected_ticks(incidents: &[(u16, u16)]) -> u32
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `affected_ticks(&[])` gives `0`.
- `affected_ticks(&[(1,4),(3,6)])` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-3000/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-3000`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
