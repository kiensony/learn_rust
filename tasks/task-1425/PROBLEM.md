# task-1425 — Remaining budget · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remaining_budget_changes(&[])` gives `vec![]`.
- `remaining_budget_changes(&[4])` gives `vec![18u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1425/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1425`.

[Module reading](https://doc.rust-lang.org/std/time/)
