# task-2915 — Latency improvement · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the improvement in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_improvement_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `latency_improvement_changes(&[])` gives `vec![]`.
- `latency_improvement_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2915/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2915`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
