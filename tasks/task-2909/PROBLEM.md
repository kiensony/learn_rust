# task-2909 — Latency increases · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe the increase in latency, saturating at zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn latency_increases_longest_run(input: &[u8]) -> u64
```

## Public examples

- `latency_increases_longest_run(&[])` gives `0`.
- `latency_increases_longest_run(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2909/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2909`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
