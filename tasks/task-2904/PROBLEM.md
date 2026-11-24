# task-2904 — Latency increases · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the increase in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Keep the first observation and every later one strictly above all earlier observations. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_increases_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `latency_increases_peaks(&[])` gives `vec![]`.
- `latency_increases_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2904/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2904`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
