# task-1423 — Remaining budget · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the remaining milliseconds after spending 50 milliseconds, saturating at zero.

Return Some((minimum, maximum)) or None for an empty trace. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn remaining_budget_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remaining_budget_bounds(&[])` gives `None`.
- `remaining_budget_bounds(&[4])` gives `Some((18u32, 18u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1423/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1423`.

[Module reading](https://doc.rust-lang.org/std/time/)
