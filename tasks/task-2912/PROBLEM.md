# task-2912 — Latency improvement · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the improvement in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return the sum of all observations, widening each one before adding. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_improvement_total(input: &[u8]) -> u64
```

## Public examples

- `latency_improvement_total(&[])` gives `0`.
- `latency_improvement_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2912/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2912`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
