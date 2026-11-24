# task-2913 — Latency improvement · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the improvement in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return Some((minimum, maximum)) or None for an empty trace. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_improvement_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `latency_improvement_bounds(&[])` gives `None`.
- `latency_improvement_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2913/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2913`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
