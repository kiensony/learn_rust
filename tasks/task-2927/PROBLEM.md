# task-2927 — Failure threshold · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if the old latency was below 100 and the new one reaches or exceeds 100, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return distinct observations in ascending order. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn failure_threshold_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `failure_threshold_distinct(&[])` gives `vec![]`.
- `failure_threshold_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2927/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2927`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
