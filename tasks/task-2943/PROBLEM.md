# task-2943 — Rollback triggers · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if new latency exceeds double the old, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return Some((minimum, maximum)) or None for an empty trace. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn rollback_triggers_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `rollback_triggers_bounds(&[])` gives `None`.
- `rollback_triggers_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2943/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2943`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
