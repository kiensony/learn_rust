# task-2945 — Rollback triggers · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if new latency exceeds double the old, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn rollback_triggers_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `rollback_triggers_changes(&[])` gives `vec![]`.
- `rollback_triggers_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2945/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2945`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
