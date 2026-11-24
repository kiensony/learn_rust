# task-2931 — Stability band · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if latencies differ by at most two ticks, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return the observations in input order, including duplicates. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn stability_band_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `stability_band_trace(&[])` gives `vec![]`.
- `stability_band_trace(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2931/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2931`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
