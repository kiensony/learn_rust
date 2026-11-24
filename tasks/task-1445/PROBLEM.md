# task-1445 — Deadline slack · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Every byte is a deterministic input clock reading in ticks of 17 milliseconds. For each reading observe the absolute difference in milliseconds from a 250-millisecond deadline.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::time::Duration without reading the wall clock. During review, point to the operation that enforces its contract.

```rust
pub fn deadline_slack_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `deadline_slack_changes(&[])` gives `vec![]`.
- `deadline_slack_changes(&[4])` gives `vec![182u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1445/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1445`.

[Module reading](https://doc.rust-lang.org/std/time/)
