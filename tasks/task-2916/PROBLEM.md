# task-2916 — Latency improvement · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the improvement in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Count adjacent pairs for which the second observation is strictly smaller. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_improvement_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `latency_improvement_descending_steps(&[])` gives `0`.
- `latency_improvement_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2916/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2916`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
