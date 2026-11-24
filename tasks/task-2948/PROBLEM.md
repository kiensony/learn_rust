# task-2948 — Failure threshold · rolling load

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if the old latency was below 100 and the new one reaches or exceeds 100, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use paired measurements, widening or saturation appropriate to the metric. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn failure_threshold_rolling_load(input: &[u8], width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `failure_threshold_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1)` gives `vec![0u64, 0u64, 0u64, 0u64, 0u64]`.
- `failure_threshold_rolling_load(&[], 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2948/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2948`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
