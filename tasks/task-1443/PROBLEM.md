# task-1443 — Deadline slack · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the absolute difference in milliseconds from a 250-millisecond deadline.

Return Some((minimum, maximum)) or None for an empty trace. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn deadline_slack_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `deadline_slack_bounds(&[])` gives `None`.
- `deadline_slack_bounds(&[4])` gives `Some((182u32, 182u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1443/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1443`.

[Module reading](https://doc.rust-lang.org/std/time/)
