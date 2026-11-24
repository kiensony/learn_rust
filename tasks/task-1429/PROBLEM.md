# task-1429 — Remaining budget · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_longest_run(input: &[u8]) -> u64
```

## Public examples

- `remaining_budget_longest_run(&[])` gives `0`.
- `remaining_budget_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1429/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1429`.

[Module reading](https://doc.rust-lang.org/std/time/)
